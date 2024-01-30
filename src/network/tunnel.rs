use std::{process, io, fs};
use std::io::{Read, Write};

use std::os::unix::io::{FromRawFd, AsRawFd};
use std::mem;

use libc::*;


#[cfg(target_os = "macos")]
const UTUN_CONTROL_NAME: &str = "com.apple.net.utun_control";

#[cfg(target_os = "macos")]
pub struct CtlInfo {
    pub ctl_id: c_uint,
    pub ctl_name: [c_uchar; 96],
}

// https://developer.apple.com/documentation/kernel/sockaddr_ctl
#[cfg(target_os = "macos")]
pub struct SocketAddressCtl {
    pub socket_len: c_uchar, // length of the structure
    pub socket_system_address: c_ushort, // AF_SYS_KERNCONTROL
    pub socket_family: c_uchar, // AF_SYSTEM
    pub socket_id: c_uint, // Controller unique identifier
    pub socket_unit: c_uint, // Kernel controller private unit number
    pub socket_reserved: [c_uint; 5], // Must be set to zero
}

pub struct Tunnel {
    handler: fs::File,
    interface_name: String,
}

impl Tunnel {

    #[cfg(target_os = "linux")]
    pub fn create(name: u8) -> Result<Tunner, io::Error> {

    }

    #[cfg(target_os = "macos")]
    pub fn create(name: u8) -> Result<Tunnel, io::Error> {
        let handle = {
            let fd = unsafe {
                socket(PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL)
            };
            if fd < 0 {
                return Err(io::Error::last_os_error());
            }
            unsafe {
                fs::File::from_raw_fd(fd)
            }
        };

        let mut info = CtlInfo {
            ctl_id: 0,
            ctl_name: {
                let mut buffer = [0u8; 96];
                buffer[..UTUN_CONTROL_NAME.len()].clone_from_slice(UTUN_CONTROL_NAME.as_bytes());
                buffer
            },
        };

        let res = unsafe {
            ioctl(handle.as_raw_fd(), CTLIOCGINFO, &mut info)
        };
        if res != 0 {
            return Err(io::Error::last_os_error());
        }

        let address = SocketAddressCtl {
            socket_len: mem::size_of::<SocketAddressCtl>() as c_uchar,
            socket_system_address: SYSPROTO_CONTROL as c_ushort,
            socket_family: AF_SYSTEM as c_uchar,
            socket_id: info.ctl_id,
            socket_unit: name as c_uint,
            socket_reserved: [0; 5],
        };

        let res = unsafe {
            let address_ptr = &address as *const SocketAddressCtl as *const c_void;
            connect(
                handle.as_raw_fd(),
                address_ptr as *const sockaddr,
                mem::size_of::<SocketAddressCtl>() as u32
            )
        };
        if res != 0 {
            return Err(io::Error::last_os_error());
        }

        let mut name_buffer = [0u8; 64];
        let mut name_length: sockent_len =


        let tun = Tunnel {
            handler: handle,
            interface_name: String::from(""),
        };
        Ok(tun)
    }

}