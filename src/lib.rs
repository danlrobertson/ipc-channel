// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(any(feature = "force-inprocess", target_os = "windows", target_os = "android", target_os = "ios"),
			feature(mpsc_select))]
#![cfg_attr(all(feature = "unstable", test), feature(specialization))]

//! An implementation of the Rust channel API (a form of communicating sequential
//! processes, CSP) over the native OS abstractions. Under the hood, this API uses
//! Mach ports on Mac and file descriptor passing over Unix sockets on Linux. The
//! serde library is used to serialize values for transport over the wire.
//!
//! For more detail, see [IpcReceiver].
//!
//! # Features
//! ## `force-inprocess`
//!
//! Force the `inprocess` backend to be used instead of the OS specific backend.
//!
//! ## `memfd`
//!
//! Use [memfd_create] to back [OsIpcSharedMemory] on linux. [memfd_create] was 
//! introduced in version 3.17. __WARNING:__ Enabling this feature with kernel
//! version less than 3.17 will cause panics on any use of [IpcSharedMemory].
//!
//! ## `unstable`
//!
//! [IpcReceiver]: ipc/struct.IpcReceiver.html
//! [IpcSharedMemory]: ipc/struct.IpcSharedMemory.html
//! [OsIpcSharedMemory]: platform/struct.OsIpcSharedMemory.html
//! [memfd_create]: http://man7.org/linux/man-pages/man2/memfd_create.2.html

#[macro_use]
extern crate lazy_static;

extern crate bincode;
extern crate libc;
extern crate rand;
extern crate serde;
#[cfg(any(feature = "force-inprocess", target_os = "windows", target_os = "android", target_os = "ios"))]
extern crate uuid;
#[cfg(all(not(feature = "force-inprocess"), any(target_os = "linux",
                                                target_os = "openbsd",
                                                target_os = "freebsd")))]
extern crate mio;
#[cfg(all(not(feature = "force-inprocess"), any(target_os = "linux",
                                                target_os = "openbsd",
                                                target_os = "freebsd")))]
extern crate fnv;
#[cfg(all(feature = "memfd", not(feature = "force-inprocess"),
          target_os="linux"))]
#[macro_use]
extern crate sc;

#[cfg(feature = "async")]
extern crate futures;


pub mod ipc;
pub mod platform;
pub mod router;

#[cfg(test)]
mod test;

pub use bincode::{Error, ErrorKind};
