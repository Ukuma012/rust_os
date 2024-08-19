use core::cell::RefCell;
use alloc::rc::Rc;

mod xhc_registers_operations;
mod error;
mod capability_register;
mod interrupter_set_register;
mod usb_command;
mod doorbell;
mod port;

pub struct XhcController<Register, Memory> {
    registers: Rc<RefCell<Register>>,
    // event_ring: EventRing<Register>,
    // command_ring: CommandRing<Register>,
    // waiting_ports: WaitingPorts,
    // device_manager: DeviceManager<Register, Memory>,
    allocator: Rc<RefCell<Memory>>
}

pub fn start_xhci_host_controller(xhc_mmio_base: u64) {
    // registerをセット
    // allocatorをセット

    // let mut xhc_controller = XhcController::new(
    //     registers,
    //     allocator,
    // );
    todo!()
}