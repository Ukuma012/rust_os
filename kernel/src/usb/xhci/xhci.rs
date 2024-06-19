use super::xhciregisters::XhciRegisters;
use super::mapper::IdentityMapper;

pub struct XhciController {
    pub registers: XhciRegisters<IdentityMapper>
}

impl XhciController {
    pub fn new(mut registers: XhciRegisters<IdentityMapper>) -> Self {
        // xHC Reset
        registers.reset();

        // Set up Device Context
        const DEVICE_SLOTS: u8 = 8;
        registers.write_max_device_slots_enabled(DEVICE_SLOTS);

        Self {
            registers
        }
    }
}