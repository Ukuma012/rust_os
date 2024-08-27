use core::cell::RefCell;
use core::fmt::Debug;
use alloc::rc::Rc;
use allocator::{memory_allocatable::MemoryAllocatable, pci_memory_allocator::PciMemoryAllocator};
use external_reg::{IdentityMapper, ExternalRegisters};
use transfer::event::event_ring::setup_event_ring;
use usb_command::setup_command_ring;
use xhc_registers::XhcRegisters;
use crate::class_driver::mouse::subscribable::MouseSubscribable;
use crate::{class_driver::mouse::driver::MouseDriver, xhc::device_context::setup_device_manager};
use crate::xhc::device_manager::DeviceManager;
use crate::xhc::transfer::command_ring::CommandRing;
use crate::xhc::transfer::event::event_ring::EventRing;

mod external_reg;
mod capability_register;
mod interrupter_set_register;
mod usb_command;
pub mod doorbell;
mod port;
mod config;
mod device_context;
mod allocator;
mod scratchpad_buffers_array_ptr;
mod scratchpad_buffer_ptr;
mod xhc_registers;
mod registers_operation;
pub mod device_manager;
pub mod transfer;

pub struct XhcController<Register, Memory> {
    registers: Rc<RefCell<Register>>,
    allocator: Rc<RefCell<Memory>>,
    device_manager: DeviceManager<Register, Memory>,
    command_ring: CommandRing<Register>,
    event_ring: EventRing<Register>,
    // waiting_ports: WaitingPorts,
}

impl<Register, Memory> XhcController<Register, Memory>
where 
    Register: XhcRegisters + 'static + Debug,
    Memory: MemoryAllocatable,
{
    pub fn new(
        registers: Register,
        mut allocator: Memory,
        mouse: MouseDriver,
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
        let device_manager: device_manager::DeviceManager<Register, Memory> = setup_device_manager(
            &mut registers,
            8,
            scratchpad_buffers_len,
            &mut allocator,
            mouse
        );
        // command_ring
        let command_ring = setup_command_ring(&mut registers, 32, &mut allocator);
        // event ring

        let(_, event_ring) = setup_event_ring(&mut registers, 1, 32, &mut allocator);
        
        registers.borrow_mut().run();

        Self {
            registers,
            allocator: Rc::new(RefCell::new(allocator)),
            device_manager,
            command_ring,
            event_ring,
        }
    }

    pub fn reset_port(&mut self) {
        let connect_ports = self
            .registers
            .borrow()
            .connecting_ports();

        if connect_ports.is_empty() {
            return ();
        }

        self.registers.borrow_mut().reset_port_at(connect_ports[0]);

        for port_id in connect_ports.into_iter().skip(1) {
            todo!()
        }
    }
}

pub fn start_xhci_host_controller(xhc_mmio_base: u64, mouse_subscriber: impl MouseSubscribable + 'static) -> XhcController<ExternalRegisters<IdentityMapper>, PciMemoryAllocator> {
    let registers = ExternalRegisters::new(xhc_mmio_base, IdentityMapper);
    let allocator = PciMemoryAllocator::new();

    let mut xhc_controller = XhcController::new(
        registers,
        allocator,
        MouseDriver::new(mouse_subscriber)
    );

    let _ = xhc_controller.reset_port();

    xhc_controller
}