use bytes::{Buf, BufMut, Bytes, BytesMut};
use protocol::rpc::header::{ObjT, ObjectMetadataT};

pub const BLOCK_DELIMITER: u8 = 0x66;
pub const FOOTER_MAGIC: u64 = 0x88e241b785f4cff7;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectMetadata {
    pub stream_id: u64,
    pub range_index: u32,
    pub epoch: u16,
    pub start_offset: u64,
    pub end_offset_delta: u32,
    pub data_len: u32,
    pub sparse_index: Bytes,
    pub key: Option<String>,
}

impl ObjectMetadata {
    pub fn new(stream_id: u64, range_index: u32, epoch: u16, start_offset: u64) -> Self {
        Self {
            stream_id,
            range_index,
            epoch,
            start_offset,
            end_offset_delta: 0,
            data_len: 0,
            sparse_index: Bytes::new(),
            key: None,
        }
    }

    /// Try find the position of the given offset which offset is after the position in the sparse index.
    pub fn find_bound(
        &self,
        start_offset: u64,
        end_offset: Option<u64>,
        mut size_hint: u32,
        position: Option<u32>,
    ) -> Option<(u32, u32)> {
        let object_end_offset = self.start_offset + self.end_offset_delta as u64;
        if start_offset >= object_end_offset {
            return None;
        }
        if let Some(end_offset) = end_offset {
            if end_offset <= self.start_offset {
                return None;
            }
        }

        // find bound start_position and end_position from sparse index
        let start_position = if start_offset == self.start_offset {
            0
        } else if let Some(position) = position {
            position
        } else {
            let mut position = 0;
            let mut cursor = self.sparse_index.clone();
            loop {
                if cursor.is_empty() {
                    break;
                }
                let index_end_offset = self.start_offset + cursor.get_u32() as u64;
                let index_position = cursor.get_u32();
                if index_end_offset <= start_offset {
                    position = index_position;
                } else {
                    // increment size hint by add previous sparse index range to cover start offset.
                    size_hint += index_position - position;
                }
            }
            if position == 0 {
                size_hint = self.data_len;
            }
            position
        };
        let mut end_position = self.data_len;
        let mut cursor = self.sparse_index.clone();
        loop {
            if cursor.is_empty() {
                break;
            }
            let index_end_offset = self.start_offset + cursor.get_u32() as u64;
            let index_position = cursor.get_u32();
            if let Some(end_offset) = end_offset {
                if index_end_offset >= end_offset {
                    end_position = index_position;
                    break;
                }
            }
            if index_position < start_position {
                continue;
            }
            if index_position - start_position >= size_hint {
                end_position = index_position;
                break;
            }
        }

        Some((start_position, end_position))
    }

    pub fn end_offset(&self) -> u64 {
        self.start_offset + self.end_offset_delta as u64
    }

    pub fn gen_object_key(&mut self, cluster: &str) {
        self.key = Some(gen_object_key(
            cluster,
            self.stream_id,
            self.range_index,
            self.epoch,
            self.start_offset,
        ));
    }
}

pub fn gen_object_key(
    cluster: &str,
    stream_id: u64,
    range_index: u32,
    epoch: u16,
    start_offset: u64,
) -> String {
    // reverse the ((stream_id * 31 + range_index) * 31 + start_offset) as prefix to make the object key dispersed.
    // prefix_number calculate formula references the Java hash code algorithm.
    let mut prefix_number = stream_id.overflowing_mul(31).0;
    prefix_number = (prefix_number.overflowing_add(range_index as u64).0)
        .overflowing_mul(31)
        .0;
    prefix_number = prefix_number.overflowing_add(start_offset).0;
    let prefix: String = format!("{:x}", prefix_number).chars().rev().collect();
    format!("{prefix}_{cluster}_{stream_id:x}_{range_index:x}_{epoch:x}_{start_offset:x}",)
}

impl From<&ObjectMetadataT> for ObjectMetadata {
    fn from(t: &ObjectMetadataT) -> Self {
        let sparse_index = if let Some(sparse_index) = t.sparse_index.clone() {
            Bytes::from(sparse_index)
        } else {
            Bytes::new()
        };
        ObjectMetadata {
            stream_id: 0,
            range_index: 0,
            epoch: 0,
            start_offset: t.start_offset as u64,
            end_offset_delta: t.end_offset_delta as u32,
            data_len: t.data_len as u32,
            sparse_index,
            key: Some(t.key.clone()),
        }
    }
}

impl From<ObjectMetadata> for ObjectMetadataT {
    fn from(m: ObjectMetadata) -> Self {
        let mut t = ObjectMetadataT::default();
        t.key = m.key.unwrap_or_default();
        t.start_offset = m.start_offset as i64;
        t.end_offset_delta = m.end_offset_delta as i32;
        t.sparse_index = Some(m.sparse_index.to_vec());
        t.data_len = m.data_len as i32;
        t
    }
}

impl From<&ObjT> for ObjectMetadata {
    fn from(t: &ObjT) -> Self {
        ObjectMetadata {
            stream_id: t.stream_id as u64,
            range_index: t.range_index as u32,
            epoch: t.epoch as u16,
            start_offset: t.start_offset as u64,
            end_offset_delta: t.end_offset_delta as u32,
            data_len: t.data_len as u32,
            sparse_index: t.sparse_index.clone().map(Bytes::from).unwrap_or_default(),
            key: None,
        }
    }
}

impl From<&ObjectMetadata> for ObjT {
    fn from(m: &ObjectMetadata) -> Self {
        let mut t = ObjT::default();
        t.stream_id = m.stream_id as i64;
        t.range_index = m.range_index as i32;
        t.epoch = m.epoch as i16;
        t.start_offset = m.start_offset as i64;
        t.end_offset_delta = m.end_offset_delta as i32;
        t.data_len = m.data_len as i32;
        t.sparse_index = Some(m.sparse_index.to_vec());
        t
    }
}

/// footer                => fix size, 48 bytes
///   sparse index pos    => u32
///   sparse index size   => u32
///   padding
///   magic               => u64
pub fn gen_footer(data_len: u32, index_len: u32) -> Bytes {
    let mut footer = BytesMut::with_capacity(48);
    footer.put_u32(data_len + 1 /* delimiter magic */);
    footer.put_u32(index_len);
    footer.put_bytes(0, 40 - 8);
    footer.put_u64(FOOTER_MAGIC);
    footer.freeze()
}
