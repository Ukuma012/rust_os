use core::fmt::Debug;
use core::num::NonZeroUsize;
use super::xhc_registers::XhcRegisters;

#[derive(Debug)]
pub struct ExternalRegisters<M>(pub xhci::registers::Registers<M>)
where
    M: xhci::accessor::Mapper + Clone;

impl<M> ExternalRegisters<M>
where
    M: xhci::accessor::Mapper + Clone,
 {
    pub fn new(mmio_base: u64, mapper: M) -> Self {
        let registers = unsafe { xhci::Registers::new(mmio_base.try_into().unwrap(), mapper) };

        Self(registers)
    }

    pub fn registers_mut(&mut self) -> &mut xhci::registers::Registers<M> {
        &mut self.0
    }
}


impl<M> XhcRegisters for ExternalRegisters<M> where M: xhci::accessor::Mapper + Clone + Debug {}

#[derive(Clone, Debug, Default)]
pub struct IdentityMapper;

impl xhci::accessor::Mapper for IdentityMapper {
    unsafe fn map(&mut self, phys_start: usize, _bytes: usize) -> core::num::NonZeroUsize {
        NonZeroUsize::new_unchecked(phys_start)
    }

    fn unmap(&mut self, _virt_start: usize, _bytes: usize) {}
}