use std::io::{Error, Result};

use libc::c_int;

/// RawSocket is a safe wrapper around a Linux `raw(7)` socket
pub struct RawSocket {
    inner: c_int,
}

impl RawSocket {
    pub fn new() -> Result<Self> {
        let sock_fd =
            check_err(unsafe { libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_ICMP) })?;
        Ok(Self { inner: sock_fd })
    }
}

impl Drop for RawSocket {
    fn drop(&mut self) {
        unsafe { libc::close(self.inner) };
    }
}

fn check_err(return_code: libc::c_int) -> Result<libc::c_int> {
    if return_code < 0 {
        return Err(Error::last_os_error());
    }
    Ok(return_code)
}
