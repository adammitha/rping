/// Represents the contents of an ICMP message as per [RFC 792](https://datatracker.ietf.org/doc/html/rfc792)
///
/// ```norun
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
pub struct IcmpMessage {
    msg_type: u8,
    code: u8,
    checksum: u16,
    identifier: u16,
    seq_num: u16,
    data: Option<Vec<u8>>,
}

impl IcmpMessage {
    pub fn new_request(seq_num: u16, data: Option<&[u8]>) -> Self {
        Self {
            msg_type: 8,
            code: 0,
            checksum: 8,
            identifier: 0,
            seq_num,
            data: data.map(|d| { d.to_owned() }),
        }
    }

    pub fn parse_reply(_payload: &[u8]) -> Self {
        todo!();
    }
}
