#![feature(btree_extract_if)]
#![feature(btree_cursors)]
#![feature(get_mut_unchecked)]
#![feature(async_fn_in_trait)]

pub mod error;
pub mod request;
mod stream;
pub mod stream_client;

pub use error::ReplicationError;
pub use stream_client::StreamClient;
