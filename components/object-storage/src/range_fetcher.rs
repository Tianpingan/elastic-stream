use std::rc::Rc;

use bytes::Bytes;
use store::{error::FetchError, option::ReadOptions, Store};

pub trait RangeFetcher {
    async fn fetch(
        &self,
        stream_id: u64,
        range_index: u32,
        start_offset: u64,
        end_offset: u64,
        max_size: u32,
    ) -> Result<RangeFetchResult, FetchError>;
}

pub struct RangeFetchResult {
    pub payload: Vec<Bytes>,
}

impl RangeFetchResult {
    pub fn new(payload: Vec<Bytes>) -> Self {
        Self { payload }
    }
}

pub struct DefaultRangeFetcher<S> {
    store: Rc<S>,
}

impl<S> RangeFetcher for DefaultRangeFetcher<S>
where
    S: Store + 'static,
{
    async fn fetch(
        &self,
        stream_id: u64,
        range_index: u32,
        start_offset: u64,
        end_offset: u64,
        max_size: u32,
    ) -> Result<RangeFetchResult, store::error::FetchError> {
        let read_option = ReadOptions {
            stream_id,
            range: range_index,
            offset: start_offset,
            max_offset: end_offset,
            max_wait_ms: 3000,
            max_bytes: max_size as i32,
        };
        let store_result = self.store.fetch(read_option).await?;
        let payload: Vec<Bytes> = store_result.results.into_iter().flatten().collect();
        Ok(RangeFetchResult { payload })
    }
}

impl<S> DefaultRangeFetcher<S>
where
    S: Store + 'static,
{
    pub fn new(store: Rc<S>) -> Self {
        Self { store }
    }
}
