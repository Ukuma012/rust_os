use super::device_context_index::DeviceContextIndex;
use crate::class_driver::mouse::driver::MouseDriver;
use crate::xhc::device_manager::control_pipe::request::Request;
use crate::xhc::device_manager::control_pipe::request_type::RequestType;
use crate::xhc::device_manager::device::phase1::Phase1;
use crate::{
    println,
    xhc::{
        allocator::memory_allocatable::MemoryAllocatable,
        device_manager::{control_pipe::ControlPipeTransfer, device::device_slot::DeviceSlot},
        doorbell::DoorbellExternalRegisters,
        transfer::event::target_event::TargetEvent,
    },
};
use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;
use device_map::DeviceConfig;
use phase::{InitStatus, Phase, DATA_BUFF_SIZE};
use xhci::{context::EndpointType, ring::trb::event::TransferEvent};

pub mod device_map;
mod device_slot;
mod phase;
mod phase1;
mod phase2;
mod phase3;
mod phase4;

pub struct Device<Doorbell, Memory> {
    slot_id: u8,
    phase: Box<dyn Phase<Doorbell, Memory>>,
    slot: DeviceSlot<Doorbell, Memory>,
    device_descriptor_buf: [u8; DATA_BUFF_SIZE],
}

impl<Doorbell, Memory> Device<Doorbell, Memory>
where
    Doorbell: DoorbellExternalRegisters + 'static,
    Memory: MemoryAllocatable,
{
    fn new(
        slot_id: u8,
        allocator: &Rc<RefCell<Memory>>,
        doorbell: &Rc<RefCell<Doorbell>>,
        mouse: MouseDriver,
    ) -> Self {
        let slot = DeviceSlot::new(slot_id, doorbell, allocator);

        let tr_dequeue_addr = slot.default_control_pipe().transfer_ring_base_addr();
        println!("1: {}", tr_dequeue_addr);

        let phase = Box::new(Phase1::new(mouse));

        let tr_dequeue_addr = slot.default_control_pipe().transfer_ring_base_addr();
        println!("2: {}", tr_dequeue_addr);

        Self {
            slot_id,
            phase,
            slot,
            device_descriptor_buf: [0; DATA_BUFF_SIZE],
        }
    }

    pub fn device_context_addr(&self) -> u64 {
        self.slot.device_context().device_context_addr()
    }

    pub fn input_context_addr(&self) -> u64 {
        self.slot.input_context().input_context_addr()
    }

    pub fn slot_id(&self) -> u8 {
        self.slot_id
    }

    pub fn new_with_init_default_control_pipe(
        config: DeviceConfig,
        allocator: &Rc<RefCell<Memory>>,
        doorbell: &Rc<RefCell<Doorbell>>,
        mouse: MouseDriver,
    ) -> Self {
        // この時点ですでに64 byte alignではない
        let mut device = Self::new(config.slot_id(), allocator, doorbell, mouse);

        device.slot.input_context_mut().set_enable_slot_context();

        device
            .slot
            .input_context_mut()
            .set_enable_endpoint(DeviceContextIndex::default());

        device.init_slot_context(config.parent_hub_slot_id(), config.port_speed());
        device.init_default_control_pipe(config.port_speed());

        device
    }

    pub fn start_init(&mut self) {
        let buff = self.device_descriptor_buf.as_mut_ptr();

        const DEVICE_DESCRIPTOR_TYPE: u16 = 1;
        let data_buff_addr = buff as u64;
        let len = self.device_descriptor_buf.len() as u32;

        self.slot.default_control_pipe_mut().control_in().with_data(
            Request::get_descriptor(DEVICE_DESCRIPTOR_TYPE, 0, len as u16),
            data_buff_addr,
            len,
        )
    }

    pub fn on_transfer_event_received(
        &mut self,
        transfer_event: TransferEvent,
        target_event: TargetEvent,
    ) -> InitStatus {
        let (init_status, phase) =
            self.phase
                .on_transfer_event_received(&mut self.slot, transfer_event, target_event);

        if let Some(phase) = phase {
            self.phase = phase;
        }

        init_status
    }

    pub fn on_endpoints_configured(&mut self) {
        for num in self.phase.interface_nums().unwrap() {
            let request_type = RequestType::new().with_ty(1).with_recipient(1);

            self.slot
                .default_control_pipe_mut()
                .control_out()
                .no_data(Request::set_protocol(request_type, num as u16));
        }
    }

    fn init_slot_context(&mut self, root_port_hub_id: u8, port_speed: u8) {
        let input_context = self.slot.input_context_mut();
        let slot = input_context.slot_mut();
        slot.set_root_hub_port_number(root_port_hub_id);
        slot.set_route_string(0);
        slot.set_context_entries(1);
        slot.set_speed(port_speed);
    }

    fn init_default_control_pipe(&mut self, port_speed: u8) {
        let tr_dequeue_addr = self.slot.default_control_pipe().transfer_ring_base_addr();

        println!("Problem! tr dequeue addr: {}", tr_dequeue_addr); //ここが64 byteでalignされていないといけない
                                                                   // どこで値が変化するのか

        let control = self.slot.input_context_mut();
        let default_control_pipe = control.endpoint_mut_at(DeviceContextIndex::default().value());

        default_control_pipe.set_endpoint_type(EndpointType::Control);
        default_control_pipe.set_max_packet_size(max_packet_size(port_speed));
        default_control_pipe.set_max_burst_size(0);
        default_control_pipe.set_tr_dequeue_pointer(tr_dequeue_addr); //ここでpanicしている
        default_control_pipe.set_dequeue_cycle_state();
        default_control_pipe.set_interval(0);
        default_control_pipe.set_max_primary_streams(0);
        default_control_pipe.set_mult(0);
        default_control_pipe.set_error_count(3);
    }
}

fn max_packet_size(port_speed: u8) -> u16 {
    match port_speed {
        3 => 64,
        4 => 512,
        _ => 8,
    }
}
