use bytes::Bytes;
use codec::frame::Frame;

use chrono::prelude::*;
use flatbuffers::FlatBufferBuilder;
use futures::future::join_all;
use log::{error, trace, warn};
use model::payload::Payload;
use protocol::rpc::header::{
    AppendResponse, AppendResponseArgs, AppendResultEntryArgs, ErrorCode, StatusArgs,
};
use std::{cell::UnsafeCell, fmt, rc::Rc};
use store::{error::AppendError, option::WriteOptions, AppendRecordRequest, AppendResult, Store};

use crate::{error::ServiceError, range_manager::RangeManager};

use super::util::{finish_response_builder, system_error_frame_bytes, MIN_BUFFER_SIZE};

#[derive(Debug)]
pub(crate) struct Append {
    // Layout of the request payload
    // +-------------------+-------------------+-------------------+-------------------+
    // |  AppendEntry 1    |  AppendEntry 2    |  AppendEntry 3    |        ...        |
    // +-------------------+-------------------+-------------------+-------------------+
    //
    // Layout of AppendEntry
    // +-------------------+-------------------+-------------------+------------------------------------------+
    // |  Magic Code(1B)   |  Meta Len(4B)     |       Meta        |  Payload Len(4B) | Record Batch Payload  |
    // +-------------------+-------------------+-------------------+------------------------------------------+
    payload: Bytes,
}

impl Append {
    pub(crate) fn parse_frame(request: &Frame) -> Result<Append, ErrorCode> {
        let payload = match request.payload {
            // For append frame, the payload must be a single buffer
            Some(ref buf) if buf.len() == 1 => buf.first().ok_or(ErrorCode::BAD_REQUEST)?,
            _ => {
                warn!(
                    "AppendRequest[stream-id={}] received without payload",
                    request.stream_id
                );
                return Err(ErrorCode::BAD_REQUEST);
            }
        };

        Ok(Append {
            payload: payload.clone(),
        })
    }

    fn replicated(&self) -> Result<bool, ErrorCode> {
        if let (Some(entry), _) =
            Payload::parse_append_entry(&self.payload).map_err(|_| ErrorCode::BAD_REQUEST)?
        {
            Ok(entry.offset.is_some())
        } else {
            unreachable!("Append request should at least contain one append-entry")
        }
    }

    /// Process message publish request
    ///
    /// On receiving a message publish request, it wraps the incoming request to a `Record`.
    /// The record is then moved to `Store::append`, where persistence, replication and other auxillary
    /// operations are properly performed.
    ///
    /// Once the underlying operations are completed, the `Store#append` API shall asynchronously return
    /// `Result<AppendResult, AppendError>`. The result will be rendered into the `response`.
    ///
    /// `response` - Mutable response frame reference, into which required business data are filled.
    ///
    pub(crate) async fn apply<S, M>(
        &self,
        store: Rc<S>,
        range_manager: Rc<UnsafeCell<M>>,
        response: &mut Frame,
    ) where
        S: Store,
        M: RangeManager,
    {
        match self.replicated() {
            Ok(replicated) => {
                if !replicated {
                    // TODO: replicate records for multi-writers
                    return;
                }
            }

            Err(e) => {
                error!("Failed to parse append request payload: {:?}", e);
                response.flag_system_err();
                response.header = Some(system_error_frame_bytes(e, "Bad Request"));
                return;
            }
        }

        let to_store_requests = match self.build_store_requests() {
            Ok(requests) => requests,
            Err(err_code) => {
                error!("Failed to build store requests. ErrorCode: {:?}", err_code);
                // The request frame is invalid, return a system error frame directly
                response.flag_system_err();
                response.header = Some(system_error_frame_bytes(err_code, "Invalid request"));
                return;
            }
        };

        let futures: Vec<_> = to_store_requests
            .into_iter()
            .map(|req| {
                trace!("Received append request: {}", req);
                let result = async {
                    if let Some(range) = unsafe { &mut *range_manager.get() }
                        .get_range_mut(req.stream_id, req.range_index)
                    {
                        if let Some(window) = range.window_mut() {
                            // Check write barrier to ensure that the incoming requests arrive in order.
                            // Some ServiceError is returned if the request is out of order.
                            window.check_barrier(&req)?;
                        } else {
                            warn!(
                                "Try append to a sealed range[{}#{}]",
                                req.stream_id, req.range_index
                            );
                            return Err(AppendError::RangeSealed);
                        }
                    } else {
                        warn!(
                            "Target stream/range is not found. stream-id={}, range-index={}",
                            req.stream_id, req.range_index
                        );
                        return Err(AppendError::RangeNotFound);
                    }
                    let options = WriteOptions::default();

                    // Append to store
                    let append_result = store.append(&options, req).await?;
                    Ok(append_result)
                };
                Box::pin(result)
            })
            .collect();

        let res_from_store: Vec<Result<AppendResult, AppendError>> = join_all(futures).await;

        let mut builder = FlatBufferBuilder::with_capacity(MIN_BUFFER_SIZE);
        let ok_status = protocol::rpc::header::Status::create(
            &mut builder,
            &StatusArgs {
                code: ErrorCode::OK,
                message: None,
                detail: None,
            },
        );

        let mut append_results: Vec<_> = vec![];
        for res in &res_from_store {
            match res {
                Ok(result) => {
                    let rm = unsafe { &mut *range_manager.get() };
                    if let Err(e) = rm
                        .commit(
                            result.stream_id,
                            result.range_index as i32,
                            result.offset as u64,
                            result.last_offset_delta,
                            result.bytes_len,
                        )
                        .await
                    {
                        warn!(
                            "Failed to ack offset on store completion to stream manager: {:?}",
                            e
                        );
                    }
                    let args = AppendResultEntryArgs {
                        timestamp_ms: Utc::now().timestamp(),
                        status: Some(ok_status),
                    };
                    append_results.push(protocol::rpc::header::AppendResultEntry::create(
                        &mut builder,
                        &args,
                    ));
                }
                Err(e) => {
                    // TODO: what to do with the offset on failure?
                    warn!("Failed to append records to store: {:?}", e);

                    let (err_code, error_message) = Self::convert_store_error(e);

                    let error_message_fb = Some(builder.create_string(error_message.as_str()));
                    let status = protocol::rpc::header::Status::create(
                        &mut builder,
                        &StatusArgs {
                            code: err_code,
                            message: error_message_fb,
                            detail: None,
                        },
                    );

                    let args = AppendResultEntryArgs {
                        timestamp_ms: 0,
                        status: Some(status),
                    };
                    append_results.push(protocol::rpc::header::AppendResultEntry::create(
                        &mut builder,
                        &args,
                    ));
                }
            }
        }

        let append_results_fb = builder.create_vector(&append_results);

        let res_args = AppendResponseArgs {
            throttle_time_ms: 0,
            entries: Some(append_results_fb),
            status: Some(ok_status),
        };
        let response_header = AppendResponse::create(&mut builder, &res_args);
        let res_header = finish_response_builder(&mut builder, response_header);
        response.header = Some(res_header);
    }

    fn build_store_requests(&self) -> Result<Vec<AppendRecordRequest>, ErrorCode> {
        let mut append_requests: Vec<AppendRecordRequest> = Vec::new();
        let mut pos = 0;
        while let (Some(entry), len) =
            Payload::parse_append_entry(&self.payload[pos..]).map_err(|e| {
                error!(
                    "Failed to decode append entries from payload. Cause: {:?}",
                    e
                );
                ErrorCode::BAD_REQUEST
            })?
        {
            let request = AppendRecordRequest {
                stream_id: entry.stream_id as i64,
                range_index: entry.index as i32,
                offset: entry.offset.map(|value| value as i64).unwrap_or(-1),
                len: entry.len,
                buffer: self.payload.slice(pos..pos + len),
            };

            append_requests.push(request);
            pos += len;
        }

        Ok(append_requests)
    }

    fn convert_store_error(err: &AppendError) -> (ErrorCode, String) {
        let code = match err {
            AppendError::RangeNotFound => ErrorCode::RANGE_NOT_FOUND,
            AppendError::RangeSealed => ErrorCode::RANGE_ALREADY_SEALED,
            AppendError::BadRequest => ErrorCode::BAD_REQUEST,
            // For the committed error, the client could regard it as success.
            AppendError::Committed => ErrorCode::APPEND_TO_COMMITTED_OFFSET,
            AppendError::Inflight => ErrorCode::APPEND_TO_PENDING_OFFSET,
            AppendError::OutOfOrder => ErrorCode::APPEND_TO_OVERTAKEN_OFFSET,
            // For other errors, return internal server error
            _ => ErrorCode::RS_INTERNAL_SERVER_ERROR,
        };

        (code, err.to_string())
    }
}

/// Converts ServiceError which is returned from RangeManager to AppendError.
impl From<ServiceError> for AppendError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::OffsetCommitted => AppendError::Committed,
            ServiceError::OffsetInFlight => AppendError::Inflight,
            ServiceError::OffsetOutOfOrder => AppendError::OutOfOrder,
            _ => AppendError::Internal,
        }
    }
}

impl fmt::Display for Append {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match Payload::parse_append_entries(&self.payload) {
            Err(e) => write!(
                f,
                "Failed to decode append entries from payload. Cause: {:?}",
                e
            ),
            Ok(entries) => entries.iter().try_for_each(|entry| {
                let offset = entry.offset.map(|value| value as i64).unwrap_or(-1);
                write!(
                    f,
                    "AppendEntry: stream-id={}, range-index={}, offset={}, len={}",
                    entry.stream_id, entry.index, offset, entry.len
                )
            }),
        }
    }
}
