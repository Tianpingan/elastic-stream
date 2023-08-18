use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
};

use model::Batch;

#[derive(Clone, Debug)]
pub struct AppendRecordRequest {
    /// Stream ID
    pub stream_id: u64,

    /// Range index
    pub range_index: i32,

    /// Base offset of the nested record entries in `buffer`
    pub offset: u64,

    /// Number of nested record entries included in `buffer`.
    pub len: u32,

    /// Buffer of a complete AppendEntry.
    ///
    /// # Layout
    /// +-------------------+-------------------+-------------------+------------------------------------------+
    /// |  Magic Code(1B)   |  Meta Len(4B)     |       Meta        |  Payload Len(4B) | Record Batch Payload  |
    /// +-------------------+-------------------+-------------------+------------------------------------------+
    pub buffer: bytes::Bytes,
}

impl Display for AppendRecordRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AppendRecordRequest {{ stream_id: {}, range_index: {}, offset: {}, len: {} }}",
            self.stream_id, self.range_index, self.offset, self.len
        )
    }
}

impl Batch for AppendRecordRequest {
    fn offset(&self) -> u64 {
        self.offset
    }

    fn len(&self) -> u32 {
        self.len
    }
}

impl PartialEq for AppendRecordRequest {
    fn eq(&self, other: &Self) -> bool {
        self.stream_id == other.stream_id
            && self.range_index == other.range_index
            && self.offset == other.offset
    }
}

impl PartialOrd for AppendRecordRequest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for AppendRecordRequest {}

impl Ord for AppendRecordRequest {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.stream_id.cmp(&self.stream_id) {
            Ordering::Equal => {}
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
        }

        match other.range_index.cmp(&self.range_index) {
            Ordering::Equal => {}
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
        }

        other.offset.cmp(&self.offset)
    }
}

#[cfg(test)]
mod tests {

    use bytes::Bytes;
    use std::{collections::BinaryHeap, error::Error};

    use super::AppendRecordRequest;

    #[test]
    fn test_order() -> Result<(), Box<dyn Error>> {
        let mut requests = BinaryHeap::new();

        let buffer = Bytes::from_static(b"test");

        let req1 = AppendRecordRequest {
            stream_id: 0,
            range_index: 0,
            offset: 0,
            len: 2,
            buffer: buffer.clone(),
        };

        let req2 = AppendRecordRequest {
            stream_id: 0,
            range_index: 0,
            offset: 2,
            len: 2,
            buffer: buffer.clone(),
        };

        requests.push(req2);
        requests.push(req1.clone());

        assert_eq!(Some(req1), requests.pop());
        Ok(())
    }
}
