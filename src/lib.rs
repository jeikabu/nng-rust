/*!

## Examples

```rust
use nng_sys::*;
use std::{ffi::CString, os::raw::c_char, ptr::null_mut};

fn example() {
    unsafe {
        let url = CString::new("inproc://nng_sys/tests/example").unwrap();
        let url = url.as_bytes_with_nul().as_ptr() as *const c_char;

        // Reply socket
        let mut rep_socket = nng_socket::default();
        nng_rep0_open(&mut rep_socket);
        nng_listen(rep_socket, url, null_mut(), 0);

        // Request socket
        let mut req_socket = nng_socket::default();
        nng_req0_open(&mut req_socket);
        nng_dial(req_socket, url, null_mut(), 0);

        // Send message
        let mut req_msg: *mut nng_msg = null_mut();
        nng_msg_alloc(&mut req_msg, 0);
        // Add a value to the body of the message
        let val = 0x12345678;
        nng_msg_append_u32(req_msg, val);
        nng_sendmsg(req_socket, req_msg, 0);

        // Receive it
        let mut recv_msg: *mut nng_msg = null_mut();
        nng_recvmsg(rep_socket, &mut recv_msg, 0);
        // Remove our value from the body of the received message
        let mut recv_val: u32 = 0;
        nng_msg_trim_u32(recv_msg, &mut recv_val);
        assert_eq!(val, recv_val);
        // Can't do this because nng uses network order (big-endian)
        //assert_eq!(val, *(nng_msg_body(recv_msg) as *const u32));

        nng_close(req_socket);
        nng_close(rep_socket);
    }
}
```
 */

// Suppress the flurry of warnings caused by using "C" naming conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// Disable clippy since this is all bindgen generated code
#![allow(clippy::all)]
#![cfg_attr(feature = "no_std", no_std)]

// Either bindgen generated source, or the static copy
#[cfg(feature = "build-bindgen")]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
#[cfg(not(feature = "build-bindgen"))]
mod bindings;

pub use crate::bindings::*;

impl nng_pipe {
    pub const NNG_PIPE_INITIALIZER: nng_pipe = nng_pipe {
        _bindgen_opaque_blob: 0,
    };
}

impl nng_socket {
    pub const NNG_SOCKET_INITIALIZER: nng_socket = nng_socket {
        _bindgen_opaque_blob: 0,
    };
}

impl nng_dialer {
    pub const NNG_DIALER_INITIALIZER: nng_dialer = nng_dialer {
        _bindgen_opaque_blob: 0,
    };
}

impl nng_listener {
    pub const NNG_LISTENER_INITIALIZER: nng_listener = nng_listener {
        _bindgen_opaque_blob: 0,
    };
}

impl nng_ctx {
    pub const NNG_CTX_INITIALIZER: nng_ctx = nng_ctx {
        _bindgen_opaque_blob: 0,
    };
}

impl nng_stat_type_enum {
    // TODO: 1.33/1.34 replace this with TryFrom once stabilized:
    // https://doc.rust-lang.org/std/convert/trait.TryFrom.html
    /// Converts value returned by [nng_stat_type](https://nanomsg.github.io/nng/man/v1.1.0/nng_stat_type.3) into `nng_stat_type_enum`.
    pub fn try_from(value: i32) -> Result<Self, TryFromIntError> {
        use crate::nng_stat_type_enum::*;
        match value {
            value if value == NNG_STAT_SCOPE as i32 => Ok(NNG_STAT_SCOPE),
            value if value == NNG_STAT_LEVEL as i32 => Ok(NNG_STAT_LEVEL),
            value if value == NNG_STAT_COUNTER as i32 => Ok(NNG_STAT_COUNTER),
            value if value == NNG_STAT_STRING as i32 => Ok(NNG_STAT_STRING),
            value if value == NNG_STAT_BOOLEAN as i32 => Ok(NNG_STAT_BOOLEAN),
            value if value == NNG_STAT_ID as i32 => Ok(NNG_STAT_ID),
            _ => Err(TryFromIntError),
        }
    }
}

impl nng_unit_enum {
    // TODO: 1.33/1.34 replace this with TryFrom once stabilized:
    // https://doc.rust-lang.org/std/convert/trait.TryFrom.html
    /// Converts value returned by [nng_stat_unit](https://nanomsg.github.io/nng/man/v1.1.0/nng_stat_unit.3) into `nng_unit_enum`.
    pub fn try_from(value: i32) -> Result<Self, TryFromIntError> {
        use crate::nng_unit_enum::*;
        match value {
            value if value == NNG_UNIT_NONE as i32 => Ok(NNG_UNIT_NONE),
            value if value == NNG_UNIT_BYTES as i32 => Ok(NNG_UNIT_BYTES),
            value if value == NNG_UNIT_MESSAGES as i32 => Ok(NNG_UNIT_MESSAGES),
            value if value == NNG_UNIT_MILLIS as i32 => Ok(NNG_UNIT_MILLIS),
            value if value == NNG_UNIT_EVENTS as i32 => Ok(NNG_UNIT_EVENTS),
            _ => Err(TryFromIntError),
        }
    }
}

impl nng_pipe_ev {
    // TODO: 1.33/1.34 replace this with TryFrom once stabilized:
    // https://doc.rust-lang.org/std/convert/trait.TryFrom.html
    pub fn try_from(value: i32) -> Result<Self, TryFromIntError> {
        use crate::nng_pipe_ev::*;
        match value {
            value if value == NNG_PIPE_EV_ADD_PRE as i32 => Ok(NNG_PIPE_EV_ADD_PRE),
            value if value == NNG_PIPE_EV_ADD_POST as i32 => Ok(NNG_PIPE_EV_ADD_POST),
            value if value == NNG_PIPE_EV_REM_POST as i32 => Ok(NNG_PIPE_EV_REM_POST),
            _ => Err(TryFromIntError),
        }
    }
}

impl nng_sockaddr_family {
    pub fn try_from(value: i32) -> Result<Self, TryFromIntError> {
        use crate::nng_sockaddr_family::*;
        match value {
            value if value == NNG_AF_UNSPEC as i32 => Ok(NNG_AF_UNSPEC),
            value if value == NNG_AF_INPROC as i32 => Ok(NNG_AF_INPROC),
            value if value == NNG_AF_IPC as i32 => Ok(NNG_AF_IPC),
            value if value == NNG_AF_INET as i32 => Ok(NNG_AF_INET),
            value if value == NNG_AF_INET6 as i32 => Ok(NNG_AF_INET6),
            value if value == NNG_AF_ZT as i32 => Ok(NNG_AF_ZT),
            _ => Err(TryFromIntError),
        }
    }
}

// TODO: 1.33/1.34 replace this with TryFrom once stabilized:
// https://doc.rust-lang.org/std/num/struct.TryFromIntError.html
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TryFromIntError;

#[cfg(not(feature = "no_std"))]
impl std::fmt::Display for TryFromIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TryFromIntError")
    }
}

#[cfg(not(feature = "no_std"))]
impl std::error::Error for TryFromIntError {}
