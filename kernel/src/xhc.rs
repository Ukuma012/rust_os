use core::cell::RefCell;
use core::fmt::Debug;
use alloc::rc::Rc;
use allocator::{memory_allocatable::MemoryAllocatable, pci_memory_allocator::PciMemoryAllocator};
use external_reg::{IdentityMapper, ExternalRegisters};
use transfer::event::event_ring::setup_event_ring;
use transfer::event::event_trb::EventTrb;
use transfer::event::target_event::TargetEvent;
use transfer::trb_raw_data::TrbRawData;
use usb_command::setup_command_ring;
use xhc_registers::XhcRegisters;
use xhci::ring::trb::event::{CommandCompletion, TransferEvent};
use crate::class_driver::mouse::subscribable::MouseSubscribable;
use crate::println;
use crate::{class_driver::mouse::driver::MouseDriver, xhc::device_context::setup_device_manager};
use crate::xhc::device_manager::DeviceManager;
use crate::xhc::transfer::command_ring::CommandRing;
use crate::xhc::transfer::event::event_ring::EventRing;
use crate::xhc::waiting_ports::WaitingPorts;
use xhci::ring::trb::event::PortStatusChange;

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
pub mod waiting_ports;

pub struct XhcController<Register, Memory> {
    registers: Rc<RefCell<Register>>,
    allocator: Rc<RefCell<Memory>>,
    device_manager: DeviceManager<Register, Memory>,
    command_ring: CommandRing<Register>,
    event_ring: EventRing<Register>,
    waiting_ports: WaitingPorts,
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

        let command_ring = setup_command_ring(&mut registers, 32, &mut allocator);

        let(_, event_ring) = setup_event_ring(&mut registers, 1, 32, &mut allocator);
        
        registers.borrow_mut().run();

        Self {
            registers,
            allocator: Rc::new(RefCell::new(allocator)),
            device_manager,
            command_ring,
            event_ring,
            waiting_ports: WaitingPorts::default()
        }
    }

    pub fn reset_port(&mut self) {
        let connect_ports = self
            .registers
            .borrow()
            .connecting_ports();

        if connect_ports.is_empty() {
            return;
        }

        self.registers.borrow_mut().reset_port_at(connect_ports[0]);

        for port_id in connect_ports.into_iter().skip(1) {
            self.waiting_ports.push(port_id);
        }
    }

    fn process_all_events(&mut self) {
        // マウスやキーボードの動作に伴ってイベントがxHCという形で溜まっていく
        // それをwhileの中で繰り返し処理していく
        // イベントの有無を能動的に調べるのでポーリング方式
        while self.event_ring.has_front() {
            self.process_event();
        }
    }

    fn process_event(&mut self) {
        if let Some(event_trb) = self.event_ring.read_event_trb() {
            return self.on_event(event_trb)
        }
    }

    fn on_event(&mut self, event_trb: EventTrb) {
        match event_trb {
            EventTrb::TransferEvent { transfer_event, target_event } => {
                self.on_transfer_event(transfer_event, target_event);
            }
            EventTrb::CommandCompletionEvent(completion) => {
                self.process_completion_event(completion)
            }
            EventTrb::PortStatusChangeEvent(port_status) => {
                self.on_port_status_change(port_status)
            }
            EventTrb::NotSupport { .. } => {}
        };

        self.event_ring.next_dequeue_pointer()
    }

    fn on_transfer_event(&mut self, transfer_event: TransferEvent, target_event: TargetEvent) {
        let slot_id = transfer_event.slot_id();

        let is_init = self.device_manager.process_transfer_event(slot_id, transfer_event, target_event);

        if is_init {
            self.configure_endpoint(slot_id);
        }
    }

    fn configure_endpoint(&mut self, slot_id: u8) {
        let input_context_addr = self.device_manager.device_slot_at(slot_id).input_context_addr();

        self.command_ring.push_configure_endpoint(input_context_addr, slot_id);
    }

    fn process_completion_event(&mut self, completion: CommandCompletion) {
        match TrbRawData::from_addr(completion.command_trb_pointer())
            .template()
            .trb_type()
            {
                // Enable Slot
                9 => self.address_device(completion),

                // Address Device
                11 => self.init_device(completion),

                // Configure Endpoint
                12 => self.device_manager.configure_endpoint(completion.slot_id()),

                _ => ()
            }
    }

    fn address_device(&mut self, completion: CommandCompletion) {
        let input_context_addr = self.device_manager.address_device(completion.slot_id(), &self.allocator);

        self.command_ring.push_address_command(input_context_addr, completion.slot_id())
    }

    fn init_device(&mut self, completion: CommandCompletion) {
        self.reset_waiting_port_if_need();

        self.device_manager.start_initialize_at(completion.slot_id());
    }

    fn reset_waiting_port_if_need(&mut self) {
        if let Some(port_id) = self.waiting_ports.pop() {
            self.registers.borrow_mut().reset_port_at(port_id)
        }
    }

    fn on_port_status_change(&mut self, port_status: PortStatusChange) {
        let port_id = port_status.port_id();

        if self.device_manager.is_addressing_port(port_id) {
            self.enable_slot(port_id);
        } else {
            self.waiting_ports.push(port_id)
        }
    }

    fn enable_slot(&mut self, port_id: u8) {
        self.registers.borrow_mut().clear_port_reset_change_at(port_id);

        self.device_manager.set_addressing_port_id(port_id);

        self.command_ring.push_enable_slot();
    }
}

pub fn start_xhci_host_controller(xhc_mmio_base: u64, mouse_subscriber: impl MouseSubscribable + 'static) {
    let mut xhc_controller = start_xhc_controller(xhc_mmio_base, mouse_subscriber);

    xhc_controller.process_all_events()
}

fn start_xhc_controller(xhc_mmio_base: u64, mouse_subscriber: impl MouseSubscribable + 'static) -> XhcController<ExternalRegisters<IdentityMapper>, PciMemoryAllocator> {
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