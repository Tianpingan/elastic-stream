use super::AlignedBuf;
use crate::error::StoreError;
use log::{error, trace};
use std::{collections::VecDeque, ptr, slice, sync::Arc};

pub(crate) struct AlignedBufWriter {
    /// Write cursor in the WAL space.
    ///
    /// # Note
    ///
    /// `cursor` points to position where next write should go. It is not necessarily aligned.
    /// After each write, it should be updated.
    pub(crate) cursor: u64,

    pub(crate) alignment: usize,

    /// Aligned buffers that are full of application data
    full: Vec<Arc<AlignedBuf>>,

    /// Aligned buffer that is currently being appended to
    current: Option<Arc<AlignedBuf>>,

    /// Pre-allocated buffers
    allocated: VecDeque<Arc<AlignedBuf>>,

    /// Flag availability of buffered data to write and flush
    buffering: bool,
}

impl AlignedBufWriter {
    pub(crate) fn new(cursor: u64, alignment: usize) -> Self {
        Self {
            cursor,
            alignment,
            full: vec![],
            current: None,
            allocated: VecDeque::new(),
            buffering: false,
        }
    }

    /// Reset writer cursor after recovery procedure
    pub(crate) fn reset_cursor(&mut self, cursor: u64) {
        self.cursor = cursor;
    }

    /// Rebase buffer during recovery and prior to writing new records.
    ///
    /// # Note
    /// The last page of WAL may be partially committed on restart. As a result,
    /// we need to reuse the buffer when appending new records.
    ///
    /// Must reset writer cursor prior to rebasing last aligned buffer.
    pub(crate) fn rebase_buf(&mut self, buf: Arc<AlignedBuf>) {
        debug_assert!(
            self.full.is_empty(),
            "BufWriter should have no full buffers"
        );

        debug_assert!(
            self.allocated.is_empty(),
            "BufWriter should have not allocated any buffers"
        );
        debug_assert_eq!(self.cursor, buf.wal_offset + buf.limit() as u64);

        self.current = Some(buf);
    }

    /// Return max WAL offset that has been allocated.
    pub(crate) fn max_allocated_wal_offset(&self) -> u64 {
        if let Some(buffer) = self.allocated.back() {
            return buffer.wal_offset + buffer.capacity as u64;
        }

        if let Some(buffer) = self.current.as_ref() {
            return buffer.wal_offset + buffer.capacity as u64;
        }

        self.cursor
    }

    /// Must invoke this method to reserve enough memory before writing.
    pub(crate) fn reserve_to(
        &mut self,
        wal_offset: u64,
        file_size: usize,
    ) -> Result<(), StoreError> {
        trace!("Reserve memory for WAL up to {}", wal_offset);

        let max_allocated_wal_offset = self.max_allocated_wal_offset();
        if wal_offset <= max_allocated_wal_offset {
            return Ok(());
        }

        {
            let prev_segment_file_index = max_allocated_wal_offset / file_size as u64;
            let segment_file_index = (wal_offset - 1) / file_size as u64;
            if prev_segment_file_index != segment_file_index {
                panic!("AlignedBuf to allocate is spanning over multiple segment files");
            }
        }

        let additional = (wal_offset - max_allocated_wal_offset) as usize;
        let mut offset = max_allocated_wal_offset;

        // First, allocate memory in alignment blocks, which we enough data to fill and then generate SQEs to submit
        // immediately.
        if additional >= self.alignment {
            let size = additional / self.alignment * self.alignment;
            let buf = AlignedBuf::new(offset, size, self.alignment)?;
            trace!(
                "Reserved {} bytes for WAL data in complete blocks. [{}, {})",
                buf.capacity,
                offset,
                offset + buf.capacity as u64
            );
            offset += buf.capacity as u64;
            self.allocated.push_back(Arc::new(buf));
        }
        debug_assert_eq!(
            offset,
            self.max_allocated_wal_offset(),
            "Max-allocated-WAL-offset should increase"
        );

        // Reserve memory block, for which we only partial data to fill.
        //
        // These partial data may be merged with future write tasks. Alternatively, we may issue stall-incurring writes if
        // configured amount of time has elapsed before collecting enough data.
        let r = additional % self.alignment;
        if 0 != r {
            let buf = AlignedBuf::new(offset, self.alignment, self.alignment)?;
            trace!(
                "Reserved {} bytes for WAL data that may only fill partial of a block. [{}, {})",
                buf.capacity,
                offset,
                offset + buf.capacity as u64
            );
            offset += buf.capacity as u64;
            self.allocated.push_back(Arc::new(buf));
        }
        debug_assert_eq!(
            offset,
            self.max_allocated_wal_offset(),
            "Max-allocated-WAL-offset should increase"
        );

        Ok(())
    }

    /// As the `current` buffer is already full and we need to move it to `full`
    /// and pop out one from `allocated` to serve further writes.
    ///
    /// If `allocated` is empty, it means we have run out of pre-allocated memory
    /// and we should return an `OutOfMemory` error.
    fn swap(&mut self) -> Result<(), StoreError> {
        if let Some(buf) = self.allocated.pop_front() {
            if let Some(prev) = self.current.replace(buf) {
                debug_assert_eq!(0, prev.remaining());
                self.full.push(prev);
            }
        } else {
            error!("AlignedBufWriter does not have enough pre-allocated buffers");
            return Err(StoreError::OutOfMemory);
        }

        Ok(())
    }

    /// Assume enough aligned memory has already been reserved.
    pub(crate) fn write(&mut self, data: &[u8]) -> Result<(), StoreError> {
        let remaining = data.len();
        let mut pos = 0;

        loop {
            if let Some(ref buf) = self.current {
                let r = buf.remaining();
                if r >= remaining - pos {
                    buf.write_buf(self.cursor + pos as u64, &data[pos..]);
                    pos += &data[pos..].len();
                    break;
                } else {
                    buf.write_buf(self.cursor + pos as u64, &data[pos..pos + r]);
                    pos += r;
                    self.swap()?;
                }
            } else {
                self.swap()?;
            }
        }
        debug_assert_eq!(pos, data.len());
        self.cursor += data.len() as u64;
        self.buffering = true;
        Ok(())
    }

    pub(crate) fn write_u32(&mut self, value: u32) -> Result<(), StoreError> {
        let big_endian = value.to_be();
        let data = ptr::addr_of!(big_endian);
        let slice = unsafe { slice::from_raw_parts(data as *const u8, std::mem::size_of::<u32>()) };
        self.write(slice)
    }

    #[allow(dead_code)]
    pub(crate) fn write_u64(&mut self, value: u64) -> Result<(), StoreError> {
        let big_endian = value.to_be();
        let data = ptr::addr_of!(big_endian);
        let slice = unsafe { slice::from_raw_parts(data as *const u8, std::mem::size_of::<u64>()) };
        self.write(slice)
    }

    /// Take backing buffers and generate submission queue entry for each of buf.
    ///
    /// If the backing buffer is full, it will be drained;
    /// If it is partially filled, its `Arc` reference will be cloned.
    ///
    /// # Arguments
    /// * `slots` - Number of io-uring SQE slots available in submission queue
    pub(crate) fn take(&mut self, slots: usize) -> Vec<Arc<AlignedBuf>> {
        let mut taken = 0;
        let mut items: Vec<_> = self
            .full
            .extract_if(|_buf| {
                taken += 1;
                taken <= slots
            })
            .collect();

        if taken < slots {
            if let Some(ref buf) = self.current {
                if buf.has_data() {
                    items.push(Arc::clone(buf));
                }
            }
            // All buffered data are taken
            self.buffering = false;
        } else if self.full.is_empty() {
            // If all buffered data are in `full` and `full` are all taken
            self.buffering = self.current.as_ref().map_or(0, |buf| buf.limit()) > 0;
        }

        items.iter().for_each(|item| {
            trace!("About to flush {} to disk", item);
        });

        items
    }

    /// Indicate if there are still some buffered data available to take and submit to io-uring
    pub(crate) fn buffering(&self) -> bool {
        self.buffering
    }

    pub(crate) fn remaining(&self) -> usize {
        self.current.as_ref().map_or(0, |buf| buf.remaining())
            + self
                .allocated
                .iter()
                .map(|buf| buf.remaining())
                .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, sync::Arc};

    use crate::io::buf::AlignedBuf;
    const ALIGNMENT: usize = 512;

    #[test]
    fn test_aligned_buf_writer() -> Result<(), Box<dyn Error>> {
        let mut buf_writer = super::AlignedBufWriter::new(0, ALIGNMENT);
        assert_eq!(0, buf_writer.cursor);
        buf_writer.reset_cursor(4100);
        let aligned_buf = AlignedBuf::new(4096, 4096, ALIGNMENT)?;
        aligned_buf.write_u32(4096, 100);
        let aligned_buf = Arc::new(aligned_buf);
        buf_writer.rebase_buf(aligned_buf);
        assert_eq!(4100, buf_writer.cursor);

        buf_writer.reserve_to(8192, 16384)?;
        buf_writer.write_u32(101)?;
        assert_eq!(4104, buf_writer.cursor);
        Ok(())
    }

    #[test]
    fn test_max_allocated_wal_offset() -> Result<(), Box<dyn Error>> {
        let mut buf_writer = super::AlignedBufWriter::new(0, ALIGNMENT);
        assert_eq!(0, buf_writer.max_allocated_wal_offset());

        let aligned_buf = Arc::new(AlignedBuf::new(1024, 512, 512)?);
        buf_writer.current = Some(Arc::clone(&aligned_buf));
        assert_eq!(buf_writer.max_allocated_wal_offset(), 1024 + 512);

        buf_writer.current = None;
        buf_writer.allocated.push_back(Arc::clone(&aligned_buf));
        assert_eq!(buf_writer.max_allocated_wal_offset(), 1024 + 512);

        let aligned_buf = Arc::new(AlignedBuf::new(1024 + 512, 512, 512)?);
        buf_writer.allocated.push_back(Arc::clone(&aligned_buf));
        assert_eq!(buf_writer.max_allocated_wal_offset(), 2048);

        Ok(())
    }

    #[test]
    fn test_reserve_to() -> Result<(), Box<dyn Error>> {
        let mut buf_writer = super::AlignedBufWriter::new(0, ALIGNMENT);
        assert_eq!(0, buf_writer.cursor);
        assert_eq!(0, buf_writer.max_allocated_wal_offset());

        buf_writer.reserve_to(ALIGNMENT as u64, 16384)?;
        assert_eq!(buf_writer.remaining(), { ALIGNMENT });
        assert_eq!(ALIGNMENT as u64, buf_writer.max_allocated_wal_offset());
        assert_eq!(0, buf_writer.cursor);

        buf_writer.reserve_to(ALIGNMENT as u64, 16384)?;
        assert_eq!(buf_writer.remaining(), { ALIGNMENT });
        assert_eq!(ALIGNMENT as u64, buf_writer.max_allocated_wal_offset());
        assert_eq!(0, buf_writer.cursor);

        buf_writer.reserve_to((ALIGNMENT + 1) as u64, 16384)?;
        assert_eq!(buf_writer.remaining(), { ALIGNMENT * 2 });
        assert_eq!(ALIGNMENT as u64 * 2, buf_writer.max_allocated_wal_offset());

        buf_writer.reserve_to(ALIGNMENT as u64 * 2 - 1, 16384)?;
        assert_eq!(buf_writer.remaining(), { ALIGNMENT * 2 });
        assert_eq!(ALIGNMENT as u64 * 2, buf_writer.max_allocated_wal_offset());
        Ok(())
    }
}
