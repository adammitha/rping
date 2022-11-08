# Design
rping(1)

Usage:
```
rping <host>
```

`host` is an IP address or domain name.

Refer to [RFC 792](https://datatracker.ietf.org/doc/html/rfc792) for the full specification of the ICMP protocol


## Constructing the ping request
`ping`-ing a host involves sending an ICMP `ECHO` request and waiting for some predetermined interval for the corresponding ICMP `ECHO` reply.

The ICMP request is encapsulated in an IP packet. The packet consists of a header and data sections:

### ICMP Message
```
Offset                
(octet)         0               1                 2             3
        +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    0   |     Type      |     Code      |          Checksum             |
        +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    4   |           Identifier          |        Sequence Number        |
        +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    8   |     Data ...
        +-+-+-+-+-
```

#### Type
8 for echo message
0 for echo reply

#### Code
0

#### Checksum
The checksum is the 16-bit ones's complement of the one's complement sum of the ICMP message starting with the ICMP Type.  For computing the checksum , the checksum field should be zero.  If the total length is odd, the received data is padded with one octet of zeros for computing the checksum.  This checksum may be replaced in the future.

#### Identifier
If code = 0, an identifier to aid in matching echos and replies, may be zero.

#### Sequence Number
If code = 0, an identifier to aid in matching echos and replies, may be zero.

#### Data
Any data received in the echo request must be returned in the response.
