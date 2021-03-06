//! This module corresponds to `mach/port.h`

use vm_types::{natural_t};

pub type mach_port_name_t = natural_t;

#[repr(C)]
pub struct ipc_port;

pub type ipc_port_t = *mut ipc_port;

pub type mach_port_t = ::libc::c_uint;

pub const MACH_PORT_NULL: mach_port_t = 0;
