#![no_std]

extern crate alloc;

#[macro_use]
extern crate log;

use axerrno::AxResult;
use core::ops::Range;

// TODO: support vgicv2
// pub(crate) mod emu_vgicdv2;
mod emu_config;
mod emu_type;

pub use emu_config::EmulatedDeviceConfig;
pub use emu_type::EmuDeviceType;

pub trait EmuDev {
    fn emu_type(&self) -> EmuDeviceType;
    fn address_range(&self) -> Range<usize>;
    fn handle_read(&self, addr: usize, width: usize) -> AxResult<usize>;
    fn handle_write(&self, addr: usize, width: usize, val: usize);
}
