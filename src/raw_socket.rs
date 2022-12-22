use nix::poll::{PollFd, PollFlags, poll};
use std::io::{Error, Result};
use std::net::SocketAddrV4;
use std::os::unix::prelude::{AsRawFd, FromRawFd, OwnedFd};
use std::ptr::addr_of;
use std::time::Duration;

/// RawSocket is a safe wrapper around a Linux `raw(7)` socket
#[derive(Debug)]
pub struct RawSocket {
    inner: OwnedFd,
}

impl RawSocket {
    pub fn new(host: &SocketAddrV4) -> Result<Self> {
        unsafe {
            let raw_socket_fd = check_err(libc::socket(
                libc::AF_INET,
                libc::SOCK_NONBLOCK | libc::SOCK_DGRAM,
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
                addr_of!(addr) as *const libc::sockaddr,
                std::mem::size_of::<libc::sockaddr>() as u32,
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

    pub fn recv(&self, buf: &mut [u8]) -> Result<isize> {
        unsafe {
            check_err(libc::recv(
                self.inner.as_raw_fd(),
                buf as *mut _ as *mut libc::c_void,
                buf.len(),
                0,
            ))
        }
    }

    pub fn poll(&self, timeout: Duration) -> Result<libc::c_int> {
        let poll_fd = PollFd::new(self.inner.as_raw_fd(), PollFlags::POLLIN);
        Ok(poll(&mut [poll_fd], timeout.as_millis().try_into().unwrap())?)
    }
}

fn check_err<T: num_traits::PrimInt>(return_code: T) -> Result<T> {
    if return_code < num_traits::Zero::zero() {
        return Err(Error::last_os_error());
    }
    Ok(return_code)
}
