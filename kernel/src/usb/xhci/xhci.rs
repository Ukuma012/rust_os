use super::xhciregisters::XhciRegisters;
use super::mapper::IdentityMapper;

pub struct XhciController {
    pub registers: XhciRegisters<IdentityMapper>
}

impl XhciController {
    pub fn new(registers: XhciRegisters<IdentityMapper>) -> Self {

        

        Self {
            registers
        }
    }
}