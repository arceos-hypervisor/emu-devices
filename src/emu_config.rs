use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::EmuDev;
use axerrno::AxResult;

// TODO: support vgicv2
//use crate::emu_vgicdv2::EmuVgicdV2;

//！Represents the configuration of an emulated device for a virtual machine.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmulatedDeviceConfig {
    pub name: String,
    /// The base IPA (Intermediate Physical Address) of the device.
    pub base_ipa: usize,
    /// The length of the device.
    pub length: usize,
    /// The IRQ (Interrupt Request) ID of the device.
    pub irq_id: usize,
    /// The type of emulated device.
    pub emu_type: usize,
    pub cfg_list: Vec<usize>,
}

/// Implementation of methods for EmuDeviceType.
impl EmulatedDeviceConfig {
    pub fn to_emu_dev(&self) -> AxResult<Arc<dyn EmuDev>> {
        //TODO: support emu devices
        /*
        match EmuDeviceType::from_usize(self.emu_type) {
            EmuDeviceType::EmuDeviceTGicdV2 => EmuVgicdV2::new_arc(&self),
            _ => panic!("emu type: {} is still not supported", self.emu_type),
        }
        */
        panic!("Still not supported");
    }
}
