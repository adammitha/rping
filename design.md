# Design
rping(1)

Usage:
```
rping <host>
```

`host` is an IP address or domain name.


## Constructing the ping request
`ping`-ing a host involves sending an ICMP `ECHO` request and waiting for some predetermined interval for the appropriate response.

The ICMP request is encapsulated in an IP packet. The packet consists of a header and data sections:

```
 Offsets       0                1               2               3
  Octet
+-------+---------------+---------------+---------------+---------------+
|   0   |      Type     |      Code     |           Checksum            |
+-------+---------------+---------------+---------------+---------------+
|   4   |         Rest of Header (varies depending on Type/Code)        |
+-------+---------------+---------------+---------------+---------------+
```

The relevant type and code for ICMP `ECHO` requests and replies are:
`ECHO` Request: Type: 8, Code: 0
`ECHO` Reply: Type: 0, Code: 0

