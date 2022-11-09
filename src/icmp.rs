use bytes::{Buf, BufMut};
use thiserror::Error;

/// Represents the contents of an ICMP message as per [`RFC 792`]
///
/// ```text
/// Offset                
/// (octet)         0               1                 2             3
///         +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///     0   |     Type      |     Code      |          Checksum             |
///         +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///     4   |           Identifier          |        Sequence Number        |
///         +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///     8   |     Data ...
///         +-+-+-+-+-
/// ```
///
/// [`RFC 792`]: https://datatracker.ietf.org/doc/html/rfc792
#[derive(Debug, PartialEq, Eq)]
pub struct IcmpMessage {
    msg_type: u8,
    code: u8,
    checksum: u16,
    identifier: u16,
    seq_num: u16,
    data: Option<Vec<u8>>,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("the provided buffer is too small to serialize this ICMP message")]
    BufTooSmall,
    #[error("the payload does not contain a complete ICMP message")]
    PayloadTooSmall,
}

pub type Result<T> = std::result::Result<T, Error>;

impl IcmpMessage {
    const ICMP_HEADER_LEN: usize = 8;

    pub fn new_request(seq_num: u16, data: Option<&[u8]>) -> Self {
        Self {
            msg_type: 8,
            code: 0,
            checksum: 0,
            identifier: 0,
            seq_num,
            data: data.map(|d| d.to_owned()),
        }
    }

    pub fn serialize_packet(&self, mut buf: &mut [u8]) -> Result<()> {
        if buf.len() < self.serialized_len() {
            return Err(Error::BufTooSmall);
        }
        buf.put_u8(self.msg_type);
        buf.put_u8(self.code);
        buf.put_u16(self.checksum);
        buf.put_u16(self.identifier);
        buf.put_u16(self.seq_num);
        if let Some(data) = self.data.as_ref() {
            buf.put(data.as_ref());
        }
        Ok(())
    }

    pub fn deserialize_packet(payload: &[u8]) -> Result<Self> {
        if payload.len() < Self::ICMP_HEADER_LEN {
            return Err(Error::PayloadTooSmall);
        }
        let data = if payload.len() > Self::ICMP_HEADER_LEN {
            Some(payload[Self::ICMP_HEADER_LEN..].as_ref().to_vec())
        } else {
            None
        };
        Ok(Self {
            msg_type: payload[0],
            code: payload[1],
            checksum: payload[2..4].as_ref().get_u16(),
            identifier: payload[4..6].as_ref().get_u16(),
            seq_num: payload[6..8].as_ref().get_u16(),
            data,
       })
    }

    fn serialized_len(&self) -> usize {
        Self::ICMP_HEADER_LEN + self.data.as_ref().map_or(0, |d| d.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_packet() {
        let msg = IcmpMessage::new_request(1, None);
        let mut buf = [0u8; 8];
        msg.serialize_packet(&mut buf).unwrap();
        assert_eq!(buf[0], 0x8);
        assert_eq!(buf[1], 0x0);
        assert_eq!(&buf[2..4], &[0x00, 0x00]);
        assert_eq!(&buf[4..6], &[0x00, 0x00]);
        assert_eq!(&buf[6..8], &[0x00, 0x01]);
    }

    #[test]
    fn test_serialize_packet_buf_too_small() {
        let msg = IcmpMessage::new_request(1, None);
        let mut buf = [0u8; 4];
        assert_eq!(msg.serialize_packet(&mut buf), Err(Error::BufTooSmall));
    }

    #[test]
    fn test_deserialize_packet() {
        let payload: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06];
        let res = IcmpMessage::deserialize_packet(&payload).unwrap();
        assert_eq!(res.msg_type, 0u8);
        assert_eq!(res.seq_num, 6u16);
    }

    #[test]
    fn test_deserialize_packet_payload_too_small() {
        let payload: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
        let res = IcmpMessage::deserialize_packet(&payload);
        assert_eq!(res, Err(Error::PayloadTooSmall));
    }
}
