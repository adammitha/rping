use std::io::{Error, Result};
use std::os::unix::prelude::FromRawFd;
use std::os::unix::prelude::OwnedFd;

/// RawSocket is a safe wrapper around a Linux `raw(7)` socket
pub struct RawSocket {
    inner: OwnedFd,
}

impl RawSocket {
    pub fn with_timeout(timeout: i64) -> Result<Self> {
        unsafe {
            let raw_socket_fd = check_err(libc::socket(
                libc::AF_INET,
                libc::SOCK_RAW,
                libc::IPPROTO_ICMP,
            ))?;
            let timeout = libc::timeval {
                tv_sec: timeout,
                tv_usec: 0,
            };
            check_err(libc::setsockopt(
                raw_socket_fd,
                libc::SOL_SOCKET,
                libc::SO_RCVTIMEO,
                &timeout as *const _ as *const libc::c_void,
                std::mem::size_of::<libc::timeval>() as u32,
            ))?;
            Ok(Self {
                inner: FromRawFd::from_raw_fd(check_err(raw_socket_fd)?),
            })
        }
    }
}

fn check_err(return_code: libc::c_int) -> Result<libc::c_int> {
    if return_code < 0 {
        return Err(Error::last_os_error());
    }
    Ok(return_code)
}
