use std::io::{Error, Result};
use std::net::SocketAddrV4;
use std::os::unix::prelude::{AsRawFd, FromRawFd, OwnedFd};

/// RawSocket is a safe wrapper around a Linux `raw(7)` socket
pub struct RawSocket {
    inner: OwnedFd,
}

impl RawSocket {
    pub fn new(timeout: i64, host: &SocketAddrV4) -> Result<Self> {
        unsafe {
            let raw_socket_fd = check_err(libc::socket(
                libc::AF_INET,
                libc::SOCK_RAW,
                libc::IPPROTO_ICMP,
            ))?;

            let addr = libc::sockaddr_in {
                sin_family: libc::AF_INET as u16,
                sin_port: u16::to_be(host.port()),
                sin_addr: libc::in_addr {
                    s_addr: u32::to_be((*host.ip()).into()),
                },
                sin_zero: Default::default(),
            };
            check_err(libc::connect(
                raw_socket_fd,
                &addr as *const _ as *const libc::sockaddr,
                std::mem::size_of::<libc::sockaddr>() as u32,
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

    pub fn send(&self, buf: &[u8]) -> Result<isize> {
        unsafe {
            check_err(libc::send(
                self.inner.as_raw_fd(),
                buf as *const _ as *const libc::c_void,
                buf.len(),
                0,
            ))
        }
    }
}

fn check_err<T: num_traits::PrimInt>(return_code: T) -> Result<T> {
    if return_code < num_traits::Zero::zero() {
        return Err(Error::last_os_error());
    }
    Ok(return_code)
}
