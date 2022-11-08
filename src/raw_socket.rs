use std::io::{Error, Result};
use std::os::unix::prelude::FromRawFd;
use std::os::unix::prelude::OwnedFd;

/// RawSocket is a safe wrapper around a Linux `raw(7)` socket
pub struct RawSocket {
    inner: OwnedFd,
}

impl RawSocket {
    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: unsafe {
                FromRawFd::from_raw_fd(check_err(libc::socket(
                    libc::AF_INET,
                    libc::SOCK_RAW,
                    libc::IPPROTO_ICMP,
                ))?)
            },
        })
    }
}

fn check_err(return_code: libc::c_int) -> Result<libc::c_int> {
    if return_code < 0 {
        return Err(Error::last_os_error());
    }
    Ok(return_code)
}
