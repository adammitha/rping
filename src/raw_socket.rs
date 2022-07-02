use libc::c_int;

pub struct RawSocket {
    inner: c_int,
}
