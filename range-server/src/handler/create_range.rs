use core::fmt;
use std::{cell::UnsafeCell, rc::Rc};

use bytes::Bytes;
use codec::frame::Frame;
use log::{error, trace, warn};
use model::range::RangeMetadata;
use protocol::rpc::header::{CreateRangeRequest, ErrorCode, RangeT, SealRangeResponseT, StatusT};
use store::Store;

use crate::range_manager::RangeManager;

use super::util::root_as_rpc_request;

#[derive(Debug)]
pub(crate) struct CreateRange<'a> {
    request: CreateRangeRequest<'a>,
}

impl<'a> CreateRange<'a> {
    pub(crate) fn parse_frame(frame: &'a Frame) -> Result<Self, ErrorCode> {
        let request = frame
            .header
            .as_ref()
            .map(|buf| root_as_rpc_request::<CreateRangeRequest>(buf))
            .ok_or(ErrorCode::BAD_REQUEST)?
            .map_err(|_e| {
                warn!(
                    "Received an invalid create range request[stream-id={}]",
                    frame.stream_id
                );
                ErrorCode::BAD_REQUEST
            })?;

        Ok(Self { request })
    }

    pub(crate) async fn apply<S, M>(
        &self,
        store: Rc<S>,
        range_manager: Rc<UnsafeCell<M>>,
        response: &mut Frame,
    ) where
        S: Store,
        M: RangeManager,
    {
        let request = self.request.unpack();

        let mut create_response = SealRangeResponseT::default();

        let manager = unsafe { &mut *range_manager.get() };

        let range = request.range;
        let range: RangeMetadata = Into::<RangeMetadata>::into(&*range);
        let mut status = StatusT::default();

        // Notify range creation to `Store`
        if let Err(e) = store.create(range.clone()).await {
            error!("Failed to create-range in store: {}", e.to_string());
            status.code = ErrorCode::RS_CREATE_RANGE;
            status.message = Some(format!("Failed to create range: {}", e));
            create_response.status = Box::new(status);
            self.build_response(response, &create_response);
            return;
        }

        // Notify range creation to `RangeManager`
        if let Err(e) = manager.create_range(range.clone()) {
            error!("Failed to create range: {:?}", e);
            // TODO: Map service error to the corresponding error code.
            status.code = ErrorCode::RS_INTERNAL_SERVER_ERROR;
            status.message = Some(format!("Failed to create range: {}", e));
            create_response.status = Box::new(status);
        } else {
            status.code = ErrorCode::OK;
            status.message = Some(String::from("OK"));
            create_response.status = Box::new(status);
            // Write back the range metadata to client so that client may maintain consistent code.
            create_response.range = Some(Box::new(Into::<RangeT>::into(&range)));
            trace!("Created range={:?}", range);
        }

        self.build_response(response, &create_response);
    }

    #[inline]
    fn build_response(&self, frame: &mut Frame, create_response: &SealRangeResponseT) {
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let resp = create_response.pack(&mut builder);
        builder.finish(resp, None);
        let data = builder.finished_data();
        frame.header = Some(Bytes::copy_from_slice(data));
    }
}

impl<'a> fmt::Display for CreateRange<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.request)
    }
}
