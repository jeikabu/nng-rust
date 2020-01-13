
Rust FFI bindings to [NNG](https://github.com/nanomsg/nng):

> NNG, like its predecessors nanomsg (and to some extent ZeroMQ), is a lightweight, broker-less library, offering a simple API to solve common recurring messaging problems, such as publish/subscribe, RPC-style request/reply, or service discovery. The API frees the programmer from worrying about details like connection management, retries, and other common considerations, so that they can focus on the application instead of the plumbing.

[![docs.rs](https://docs.rs/nng-sys/badge.svg)](https://docs.rs/nng-sys)
[![crates.io](http://img.shields.io/crates/v/nng-sys.svg)](http://crates.io/crates/nng-sys)
![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rustc 1.31+](https://img.shields.io/badge/rustc-1.31+-lightgray.svg)
[![travis](https://travis-ci.org/jeikabu/nng-rust.svg?branch=master)](https://travis-ci.org/jeikabu/nng-rust)
[![Build Status](https://dev.azure.com/jeikabu/nng-rust/_apis/build/status/jeikabu.nng-rust?branchName=master)](https://dev.azure.com/jeikabu/nng-rust/_build/latest?definitionId=1&branchName=master)

## Usage

Version of this crate tracks NNG: `<NNG_version>-rc.<crate_version>` (e.g. `1.1.1-rc.2`).

To use the __latest crate__ for the most recent __stable version of NNG__ (1.2.x), in `Cargo.toml`:  
```toml
[dependencies]
nng-sys = "1.2.4-rc"
```

Requirements:
- [cmake](https://cmake.org/) in `PATH`
    - On Linux/macOS: default generator is "Unix Makefiles"
    - On Windows: default generator is generally latest version of Visual Studio installed
- _Optional_ libclang needed if using `build-bindgen` feature to run [bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html)

## Features

- `build-nng`: use cmake to build NNG from source (enabled by default)
- `build-bindgen`: run bindgen to re-generate Rust FFI bindings to C
- `cmake-unix`: use cmake generator "Unix Makefiles" (default on Linux/macOS)
- `cmake-ninja`: use cmake generator "Ninja"
- `cmake-vs2017`: use cmake generator "Visual Studio 15 2017"
- `cmake-vs2019`: use cmake generator "Visual Studio 16 2019"
- `nng-stats`: enable NNG stats `NNG_ENABLE_STATS` (enabled by default)
- `nng-tls`: enable TLS `NNG_ENABLE_TLS` (requires mbedTLS)
- `nng-supplemental`: generate bindings to NNG's supplemental functions
- `nng-compat`: generate bindings to NNG's nanomsg compatible functions

_Example_) Re-generate FFI bindings with bindgen:
```toml
[dependencies]
nng-sys = { version = "1.2.4-rc", features = ["build-bindgen"] }
```

_Example_) Disable stats and use Ninja cmake generator:
```toml
[dependencies.nng-sys]
version = "1.2.4-rc"
default-features = false
features = ["cmake-ninja"]
```

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