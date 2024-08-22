use alloc::sync::Arc;
use core::ops::Range;

use axerrno::{ax_err, AxResult};
use arm_vgic::Vgic;

use crate::{
    EmuDeviceType,
    EmulatedDeviceConfig,
    EmuDev,
};

#[derive(Debug)]
pub struct EmuVgicdV2 {
    address_range: Range<usize>,
    vgic: Vgic
}

impl EmuDev for EmuVgicdV2 {
    fn emu_type(&self) -> EmuDeviceType {
        EmuDeviceType::EmuDeviceTGicdV2
    }

    fn address_range(&self) -> Range<usize> {
        self.address_range.clone()   
    }

    fn handle_read(&self, _addr: usize, _width: usize) -> AxResult<usize> {
        info!("emu_vgicdv2 hanlder read ");
        //self.vgicd.handler(emu_ctx)
        Ok(0)
    }
    fn handle_write(&self, _addr: usize, _width: usize, _val: usize) {
        info!("emu_vgicdv2 hanlder write ");

    }
}

impl EmuVgicdV2 {
    pub(crate) fn new_arc(emu_cfg: &EmulatedDeviceConfig) -> 
        AxResult<Arc<dyn EmuDev>> {
        if emu_cfg.cfg_list.len() != 1 {
            return ax_err!(InvalidInput, "Create EmuDeviceTGicdV2 not have vcpu
                num config");
        }
        let vcpu_num = emu_cfg.cfg_list[0];
        Ok(Arc::new(Self {
            address_range: emu_cfg.base_ipa..emu_cfg.base_ipa 
                + emu_cfg.length,
            vgic: Vgic::new(vcpu_num)
        }))
    }
}
