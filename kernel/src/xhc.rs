use core::cell::RefCell;
use core::fmt::Debug;
use alloc::rc::Rc;
use allocator::{memory_allocatable::MemoryAllocatable, pci_memory_allocator::PciMemoryAllocator};
use external_reg::{IdentityMapper, ExternalRegisters};
use xhc_registers::XhcRegisters;

mod external_reg;
mod capability_register;
mod interrupter_set_register;
mod usb_command;
mod doorbell;
mod port;
mod config;
mod device_context;
mod allocator;
mod scratchpad_buffers_array_ptr;
mod scratchpad_buffer_ptr;
mod xhc_registers;
mod registers_operation;
mod device_manager;
mod transfer;

pub struct XhcController<Register, Memory> {
    registers: Rc<RefCell<Register>>,
    allocator: Rc<RefCell<Memory>>
    // device_manager: DeviceManager<Register, Memory>,
    // event_ring: EventRing<Register>,
    // command_ring: CommandRing<Register>,
    // waiting_ports: WaitingPorts,
}

impl<Register, Memory> XhcController<Register, Memory>
where 
    Register: XhcRegisters + 'static + Debug,
    Memory: MemoryAllocatable,
{
    pub fn new(
        registers: Register,
        mut allocator: Memory
    ) -> Self {
        let mut registers = Rc::new(RefCell::new(registers));

        registers
            .borrow_mut()
            .reset();

        registers
            .borrow_mut()
            .write_max_device_slots_enabled(8);

        let scratchpad_buffers_len = registers
                                    .borrow()
                                    .read_max_scratchpad_buffers_len();

        // device manager
        // let device_manager = setup_device_manager();
        // command_ring
        // event ring
        
        registers.borrow_mut().run();

        Self {
            registers,
            allocator: Rc::new(RefCell::new(allocator))
        }
    }
}

pub fn start_xhci_host_controller(xhc_mmio_base: u64) {
    let registers = ExternalRegisters::new(xhc_mmio_base, IdentityMapper);
    let allocator = PciMemoryAllocator::new();

    let mut xhc_controller = XhcController::new(
        registers,
        allocator,
    );
    todo!()
}