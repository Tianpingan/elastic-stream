use byteorder::ReadBytesExt;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use log::{trace, warn};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use protocol::rpc::header::{CommonFlags, GoAwayFlags, OperationCode};
use std::cell::RefCell;
use std::io::Cursor;

use crate::error::FrameError;

pub(crate) const MAGIC_CODE: u8 = 23;

pub(crate) const MIN_FRAME_LENGTH: u32 = 16;

// Max frame length 16MB
pub(crate) const MAX_FRAME_LENGTH: u32 = 16 * 1024 * 1024;

thread_local! {
    static STREAM_ID: RefCell<u32> = RefCell::new(1);
}

#[derive(Debug)]
pub struct Frame {
    pub operation_code: OperationCode,

    /// Further refine semantic of the opcode.
    ///
    /// Semantics of flag is defined per opcode basis.
    flag: u8,

    /// Stream-ID, starting from 1.
    /// stream-id `0` is used as placeholder only.
    pub stream_id: u32,

    pub header_format: HeaderFormat,

    pub header: Option<Bytes>,

    pub payload: Option<Vec<Bytes>>,
}

impl Frame {
    pub fn new(op: OperationCode) -> Self {
        let stream_id = STREAM_ID.with(|f| {
            let mut value = f.borrow_mut();
            let current = *value;
            *value += 1;
            current
        });

        Self {
            operation_code: op,
            flag: 0,
            stream_id,
            header_format: HeaderFormat::FlatBuffer,
            header: None,
            payload: None,
        }
    }

    pub fn is_response(&self) -> bool {
        self.has_common_flag(CommonFlags::RESPONSE)
    }

    pub fn flag_response(&mut self) {
        self.flag_common(CommonFlags::RESPONSE);
    }

    pub fn end_of_stream(&self) -> bool {
        self.has_common_flag(CommonFlags::END_OF_STREAM)
    }

    pub fn flag_end_of_response_stream(&mut self) {
        self.flag_common(CommonFlags::RESPONSE);
        self.flag_common(CommonFlags::END_OF_STREAM);
    }

    pub fn system_error(&self) -> bool {
        self.has_common_flag(CommonFlags::SYSTEM_ERROR)
    }

    pub fn flag_system_err(&mut self) {
        self.flag_common(CommonFlags::END_OF_STREAM);
        self.flag_common(CommonFlags::RESPONSE);
        self.flag_common(CommonFlags::SYSTEM_ERROR);
    }

    #[inline]
    fn flag_common(&mut self, flag: CommonFlags) {
        self.flag |= flag.0 as u8;
    }

    #[inline]
    fn has_common_flag(&self, flag: CommonFlags) -> bool {
        self.flag & flag.0 as u8 == flag.0 as u8
    }

    #[inline]
    pub fn flag_go_away(&mut self, flag: GoAwayFlags) {
        self.flag |= flag.0 as u8;
    }

    #[inline]
    pub fn has_go_away_flag(&self, flag: GoAwayFlags) -> bool {
        self.flag & flag.0 as u8 == flag.0 as u8
    }

    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), FrameError> {
        let frame_length = match src.read_u32::<byteorder::NetworkEndian>() {
            Ok(n) => {
                trace!("Incoming frame length is: {}", n);
                n
            }
            Err(_) => {
                if src.remaining() > 0 {
                    trace!(
                        "Only {} bytes in buffer. Read more data to proceed",
                        src.remaining()
                    );
                }
                return Err(FrameError::Incomplete);
            }
        };

        if frame_length < MIN_FRAME_LENGTH {
            warn!(
                "Illegal frame length: {}, fewer than minimum: {}",
                frame_length, MIN_FRAME_LENGTH
            );
            return Err(FrameError::BadFrame(format!(
                "Length of the incoming frame is: {}, less than the minimum possible: {}",
                frame_length, MIN_FRAME_LENGTH
            )));
        }

        // Check if the frame length is legal or not.
        if frame_length > MAX_FRAME_LENGTH {
            warn!(
                "Illegal frame length: {}, greater than maximum allowed: {}",
                frame_length, MAX_FRAME_LENGTH
            );
            return Err(FrameError::TooLongFrame {
                found: frame_length,
                max: MAX_FRAME_LENGTH,
            });
        }

        // Check if the frame is complete
        if src.remaining() < frame_length as usize {
            trace!(
                "Incoming frame length: {}, remaining bytes: {}",
                frame_length,
                src.remaining()
            );
            return Err(FrameError::Incomplete);
        }

        // Verify magic code
        let magic_code = src.get_u8();
        if MAGIC_CODE != magic_code {
            warn!(
                "Illegal magic code, expecting: {}, actual: {}",
                MAGIC_CODE, magic_code
            );
            return Err(FrameError::MagicCodeMismatch {
                found: magic_code,
                expected: MAGIC_CODE,
            });
        }

        // op code
        src.advance(2);

        // flag
        src.advance(1);

        // stream id
        src.advance(4);

        // header format
        src.advance(1);

        // header length
        let header_length: u32 = src.get_u8() as u32;
        let header_length = src.get_u16() as u32 + (header_length << 16);
        if header_length > frame_length - MIN_FRAME_LENGTH {
            return Err(FrameError::BadFrame(format!(
                "Header length[{}] exceeds maximum value possible given that frame-length is {}",
                header_length, frame_length
            )));
        }
        src.advance(header_length as usize);

        let mut payload = None;
        if header_length + MIN_FRAME_LENGTH < frame_length {
            let payload_length = frame_length - header_length - MIN_FRAME_LENGTH;
            if payload_length > src.remaining() as u32 {
                return Err(FrameError::BadFrame(format!(
                    "Payload length[{}] exceeds maximum value possible given that frame-length is {} and header-length is {}",
                    payload_length,
                    frame_length,
                    header_length
                )));
            }
            let body = src.copy_to_bytes(payload_length as usize);
            payload = Some(body);
        }

        // Remaining bytes are checksum
        debug_assert!(
            src.remaining() >= 4,
            "There is at least 4 bytes in the buffer, holding checksum of the payload"
        );

        if let Some(body) = payload {
            let checksum = src.get_u32();
            let ckm = util::crc32::crc32(body.as_ref());
            if checksum != ckm {
                warn!(
                    "Payload checksum mismatch. Expecting: {}, Actual: {}",
                    checksum, ckm
                );
                return Err(FrameError::PayloadChecksumMismatch {
                    expected: checksum,
                    actual: ckm,
                });
            }
        } else {
            // checksum
            src.advance(4);
        }

        Ok(())
    }

    #[inline]
    fn to_opcode_unchecked(code: u16) -> OperationCode {
        OperationCode(code as i16)
    }

    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, FrameError> {
        // Safety: previous `check` method ensures we are having a complete frame to parse
        let frame_length = src.get_u32();
        let mut remaining = frame_length;

        // Skip magic code
        src.advance(1);
        remaining -= 1;

        let op_code = src.get_u16();
        remaining -= 2;
        let op_code = Self::to_opcode_unchecked(op_code);

        let flag = src.get_u8();
        remaining -= 1;

        let stream_id = src.get_u32();
        remaining -= 4;

        let header_format = src.get_u8();
        remaining -= 1;
        let header_format = HeaderFormat::try_from(header_format).unwrap_or(HeaderFormat::Unknown);

        let mut frame = Frame {
            operation_code: op_code,
            flag,
            stream_id,
            header_format,
            header: None,
            payload: None,
        };

        let header_length: u32 = src.get_u8() as u32;
        let header_length = src.get_u16() as u32 + (header_length << 16);
        remaining -= 3;

        if header_length > 0 {
            let header = src.copy_to_bytes(header_length as usize);
            frame.header = Some(header);
        }
        remaining -= header_length;

        let payload_length = remaining - 4;
        if payload_length > 0 {
            let payload = src.copy_to_bytes(payload_length as usize);
            frame.payload = Some(vec![payload]);
        }
        remaining -= payload_length;

        // payload checksum
        src.advance(4);
        remaining -= 4;
        debug_assert!(0 == remaining);

        Ok(frame)
    }

    pub fn encode(&self) -> Result<Vec<Bytes>, FrameError> {
        let mut encode_result = Vec::new();
        let mut frame_length = 16;
        if let Some(header) = &self.header {
            frame_length += header.len();
        }

        let payload_len = self
            .payload
            .iter()
            .flatten()
            .map(|b| b.len())
            .sum::<usize>();
        frame_length += payload_len;

        if frame_length > crate::frame::MAX_FRAME_LENGTH as usize {
            return Err(FrameError::TooLongFrame {
                found: frame_length as u32,
                max: MAX_FRAME_LENGTH,
            });
        }

        // Only store the header part in the buffer
        let mut basic_part = BytesMut::with_capacity(frame_length - payload_len);

        basic_part.put_u32(frame_length as u32);
        basic_part.put_u8(crate::frame::MAGIC_CODE);
        basic_part.put_i16(self.operation_code.0);
        basic_part.put_u8(self.flag);
        basic_part.put_u32(self.stream_id);
        basic_part.put_u8(self.header_format.into());

        if let Some(header) = &self.header {
            let bytes = (header.len() as u32).to_be_bytes();
            debug_assert!(4 == bytes.len());
            basic_part.extend_from_slice(&bytes[1..]);
            basic_part.extend_from_slice(header.as_ref());
        } else {
            basic_part.put_u8(0);
            basic_part.put_u16(0);
        }

        encode_result.push(basic_part.freeze());

        if let Some(payload) = &self.payload {
            for p in payload {
                encode_result.push(p.clone());
            }
            let checksum = util::crc32::crc32_vectored(payload.iter());
            encode_result.push(Bytes::copy_from_slice(&checksum.to_be_bytes()[..]));
        } else {
            // Dummy checksum
            encode_result.push(Bytes::from_static(&[0, 0, 0, 0]));
        }

        Ok(encode_result)
    }

    pub fn get_response_payload(&self) -> Option<Bytes> {
        if let Some(payload_parts) = &self.payload {
            if payload_parts.len() == 1 {
                // Response which passed by network payload is parsed as single bytes.
                return Some(payload_parts[0].clone());
            }
            let mut bytes =
                BytesMut::with_capacity(payload_parts.iter().map(|b| b.len()).sum::<usize>());
            for b in payload_parts {
                bytes.put(b.clone());
            }
            Some(bytes.freeze())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum HeaderFormat {
    Unknown = 0,
    // FlatBuffers format indicates that the payload of the extended header is serialized by flatbuffers.
    // This is the only supported format for now.
    FlatBuffer = 0x01,
    ProtoBuffer = 0x02,
    JSON = 0x03,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use bytes::{BufMut, BytesMut};

    use super::*;

    #[test]
    fn test_num_enum() {
        let res = HeaderFormat::try_from(1u8);
        assert_eq!(Ok(HeaderFormat::FlatBuffer), res);

        let num: u8 = HeaderFormat::JSON.into();
        assert_eq!(3, num);

        let res = Frame::to_opcode_unchecked(0u16);
        assert_eq!(OperationCode::UNKNOWN, res);
    }

    #[test]
    fn test_check() {
        let raw = [1u8];
        let mut rdr = Cursor::new(&raw[..]);
        let res = Frame::check(&mut rdr);
        assert_eq!(Err(FrameError::Incomplete), res);

        // On read failure, the cursor should be intact.
        assert_eq!(1, rdr.remaining());
    }

    #[test]
    fn test_check_min_frame_length() {
        let mut buffer = BytesMut::new();
        buffer.put_u32(10);

        let mut cursor = Cursor::new(&buffer[..]);
        match Frame::check(&mut cursor) {
            Ok(_) => {
                panic!("Should have detected the frame length issue");
            }
            Err(e) => {
                assert_eq!(
                    FrameError::BadFrame(
                        "Length of the incoming frame is: 10, less than the minimum possible: 16"
                            .to_owned()
                    ),
                    e
                );
            }
        }
    }

    #[test]
    fn test_check_max_frame_length() {
        let mut buffer = BytesMut::new();
        buffer.put_u32(MAX_FRAME_LENGTH + 1);
        let mut cursor = Cursor::new(&buffer[..]);
        match Frame::check(&mut cursor) {
            Ok(_) => {
                panic!("Should have detected the frame length issue");
            }
            Err(e) => {
                assert_eq!(
                    FrameError::TooLongFrame {
                        found: MAX_FRAME_LENGTH + 1,
                        max: MAX_FRAME_LENGTH
                    },
                    e
                );
            }
        }
    }

    #[test]
    fn test_check_magic_code() {
        let mut buffer = BytesMut::new();
        buffer.put_u32(MIN_FRAME_LENGTH);
        // magic code
        buffer.put_u8(16u8);
        // operation code
        buffer.put_i16(OperationCode::PING.0);
        // flag
        buffer.put_u8(0u8);
        // stream identifier
        buffer.put_u32(2);
        // header format + header length
        buffer.put_u32(0);
        // header
        // payload
        // payload checksum
        buffer.put_u32(0);

        let mut cursor = Cursor::new(&buffer[..]);

        match Frame::check(&mut cursor) {
            Ok(_) => {
                panic!("Should have detected the frame magic code mismatch issue");
            }
            Err(e) => {
                assert_eq!(
                    FrameError::MagicCodeMismatch {
                        found: 16u8,
                        expected: MAGIC_CODE
                    },
                    e
                );
            }
        }
    }

    #[test]
    fn test_encode_header() {
        let mut header = BytesMut::with_capacity(16);
        header.put(&b"abc"[..]);

        let frame = Frame {
            operation_code: OperationCode::PING,
            flag: 1,
            stream_id: 2,
            header_format: HeaderFormat::FlatBuffer,
            header: Some(header.freeze()),
            payload: None,
        };

        let encode_result = frame.encode();
        let mut bytes_mute = BytesMut::new();
        for ele in encode_result.unwrap() {
            bytes_mute.put_slice(&ele);
        }
        let mut buf = bytes_mute.freeze();

        let frame_length = buf.get_u32();
        assert_eq!(19, frame_length);
        assert_eq!(MAGIC_CODE, buf.get_u8());
        assert_eq!(1, buf.get_u16());
        assert_eq!(1, buf.get_u8());
        assert_eq!(2, buf.get_u32());
        assert_eq!(1, buf.get_u8());
        // header length
        assert_eq!(0, buf.get_u8());
        assert_eq!(3, buf.get_u16());

        let header = buf.copy_to_bytes(3);
        assert_eq!(b"abc", header.as_ref());

        assert_eq!(0, buf.get_u32());

        assert_eq!(0, buf.remaining());
    }

    #[test]
    fn test_encode_body() {
        let mut body = BytesMut::with_capacity(16);
        body.put(&b"abc"[..]);

        let frame = Frame {
            operation_code: OperationCode::PING,
            flag: 1,
            stream_id: 2,
            header_format: HeaderFormat::FlatBuffer,
            header: None,
            payload: Some(vec![body.freeze()]),
        };

        let encode_result = frame.encode();
        let mut bytes_mute = BytesMut::new();
        for ele in encode_result.unwrap() {
            bytes_mute.put_slice(&ele);
        }
        let mut buf = bytes_mute.freeze();

        assert_eq!(19, buf.get_u32());

        assert_eq!(MAGIC_CODE, buf.get_u8());
        assert_eq!(1, buf.get_u16());
        assert_eq!(1, buf.get_u8());
        assert_eq!(2, buf.get_u32());
        assert_eq!(1, buf.get_u8());
        // header length
        assert_eq!(0, buf.get_u8());
        assert_eq!(0, buf.get_u16());

        let body = buf.copy_to_bytes(3);
        assert_eq!(b"abc", body.as_ref());
        // checksum
        assert_eq!(util::crc32::crc32(b"abc"), buf.get_u32());
        assert_eq!(0, buf.remaining());
        assert_eq!(0, buf.len());
    }

    #[test]
    fn test_too_long_header_length() {
        let mut raw_frame = BytesMut::with_capacity(16);
        // frame length
        raw_frame.put_u32(19);
        // magic code
        raw_frame.put_u8(MAGIC_CODE);
        // operation code
        raw_frame.put_i16(OperationCode::PING.0);
        // flag
        raw_frame.put_u8(1);
        // stream identifier
        raw_frame.put_u32(2);
        // header format + header length
        raw_frame.put_u8(HeaderFormat::FlatBuffer.into());
        // Set a header length that is too long
        raw_frame.extend_from_slice((1024_i32).to_be_bytes()[1..].as_ref());
        // header
        raw_frame.put(&b"abc"[..]);
        // empty payload
        // payload checksum
        raw_frame.put_u32(0);

        let mut cursor = Cursor::new(&raw_frame[..]);
        match Frame::check(&mut cursor) {
            Ok(_) => {
                panic!("Should have detected the frame header length issue");
            }
            Err(e) => {
                assert!(matches!(e, FrameError::BadFrame { .. }));
            }
        }
    }

    #[test]
    fn test_bad_frame_no_checksum() {
        let mut raw_frame = BytesMut::with_capacity(16);
        // frame length
        raw_frame.put_u32(25);
        // magic code
        raw_frame.put_u8(MAGIC_CODE);
        // operation code
        raw_frame.put_i16(OperationCode::PING.0);
        // flag
        raw_frame.put_u8(1);
        // stream identifier
        raw_frame.put_u32(2);
        // header format + header length
        raw_frame.put_u8(HeaderFormat::FlatBuffer.into());
        // header length
        raw_frame.extend_from_slice((10_i32).to_be_bytes()[1..].as_ref());
        // header
        raw_frame.put(&b"header"[..]);
        // payload length
        // payload
        raw_frame.put(&b"abc"[..]);
        // payload checksum
        raw_frame.put_u32(0);

        let mut cursor = Cursor::new(&raw_frame[..]);

        match Frame::check(&mut cursor) {
            Ok(_) => {
                panic!("Should have detected the frame payload length issue");
            }
            Err(e) => {
                assert!(matches!(e, FrameError::BadFrame { .. }));
            }
        }
    }
    #[test]
    fn test_check_and_parse() {
        let mut header = BytesMut::new();
        header.put(&b"header"[..]);

        let mut body = BytesMut::with_capacity(16);
        body.put(&b"abc"[..]);

        let frame = Frame {
            operation_code: OperationCode::PING,
            flag: 1,
            stream_id: 2,
            header_format: HeaderFormat::FlatBuffer,
            header: Some(header.freeze()),
            payload: Some(vec![body.freeze()]),
        };

        let encode_result = frame.encode();
        let mut bytes_mute = BytesMut::new();
        for ele in encode_result.unwrap() {
            bytes_mute.put_slice(&ele);
        }
        bytes_mute.put_slice("dummy".as_bytes());
        let buf = bytes_mute.freeze();

        assert_eq!(29 + 5, buf.remaining());

        let mut cursor = Cursor::new(&buf[..]);

        // Frame::check should pass
        assert_eq!(Ok(()), Frame::check(&mut cursor));

        // Reset cursor
        cursor.set_position(0);

        // Validate parse
        let decoded = Frame::parse(&mut cursor).unwrap();
        assert_eq!(OperationCode::PING, decoded.operation_code);
        assert_eq!(1, decoded.flag);
        assert_eq!(2, decoded.stream_id);
        assert_eq!(HeaderFormat::FlatBuffer, decoded.header_format);
        assert_eq!(Some(Bytes::from("header")), decoded.header);
        assert_eq!(Some(vec![Bytes::from("abc")]), decoded.payload);
    }
}
