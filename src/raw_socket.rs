use std::io::{Error, Result};
use std::os::unix::io::RawFd;

/// RawSocket is a safe wrapper around a Linux `raw(7)` socket
pub struct RawSocket {
    inner: RawFd,
}

impl RawSocket {
    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: check_err(unsafe {
                libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_ICMP)
            })?,
        })
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
