use std::time::Duration;

use bytes::{Bytes, BytesMut};
use protocol::rpc::header::{HeartbeatRequestT, ListRangesRequestT, RangeCriteriaT};

use crate::{client_role::ClientRole, data_node::DataNode, range_criteria::RangeCriteria};

#[derive(Debug)]
pub enum Request {
    Heartbeat {
        client_id: String,
        role: ClientRole,
        data_node: Option<DataNode>,
    },

    ListRanges {
        timeout: Duration,
        criteria: Vec<RangeCriteria>,
    },
}

impl From<&Request> for Bytes {
    fn from(request: &Request) -> Self {
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        match request {
            Request::Heartbeat {
                client_id,
                role,
                data_node,
            } => {
                let data_node = data_node.as_ref().map(|node| Box::new(node.into()));
                let mut heartbeat_request = HeartbeatRequestT::default();
                heartbeat_request.client_id = Some(client_id.to_owned());
                heartbeat_request.client_role = role.into();
                heartbeat_request.data_node = data_node;
                let heartbeat = heartbeat_request.pack(&mut builder);
                builder.finish(heartbeat, None);
            }

            Request::ListRanges { timeout, criteria } => {
                let list: Vec<_> = criteria
                    .into_iter()
                    .map(|c| {
                        let mut criteria = RangeCriteriaT::default();
                        match c {
                            RangeCriteria::StreamId(stream_id) => {
                                criteria.stream_id = *stream_id;
                            }
                            RangeCriteria::DataNode(data_node) => {
                                criteria.data_node = Some(Box::new(data_node.into()));
                            }
                        };
                        criteria
                    })
                    .collect();
                let mut request = ListRangesRequestT::default();
                request.timeout_ms = timeout.as_millis() as i32;
                request.range_criteria = Some(list);
                let req = request.pack(&mut builder);
                builder.finish(req, None);
            }
        };
        let buf = builder.finished_data();
        let mut buffer = BytesMut::with_capacity(buf.len());
        buffer.extend_from_slice(buf);
        buffer.freeze()
    }
}