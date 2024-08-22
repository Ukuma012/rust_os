use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::{class_driver::mouse::driver::MouseDriver, xhc::{allocator::memory_allocatable::MemoryAllocatable, device_manager::descriptor::Descriptor, doorbell::DoorbellExternalRegisters}};
use crate::xhc::device_manager::device::Phase;
use crate::xhc::device_manager::descriptor::structs::configuration_descriptor::ConfigurationDescriptor;
use crate::xhc::device_manager::descriptor::descriptor_sequence::DescriptorSequence;
use crate::xhc::device_manager::device::InitStatus;
use crate::xhc::device_manager::descriptor::hid::HidDeviceDescriptors;
use crate::xhc::device_manager::descriptor::structs::interface_descriptor::InterfaceDescriptor;
use crate::xhc::device_manager::control_pipe::request::Request;
use crate::xhc::device_manager::control_pipe::ControlPipe;
use crate::xhc::device_manager::control_pipe::ControlPipeTransfer;
use super::phase3::Phase3;


pub struct Phase2 {
    mouse: MouseDriver,
}

impl Phase2 {
    pub const fn new(mouse: MouseDriver) -> Self {
        Self {
            mouse
        }
    }
}

impl<Doorbell, Memory> Phase<Doorbell, Memory> for Phase2
where 
    Memory: MemoryAllocatable,
    Doorbell: DoorbellExternalRegisters + 'static,
{
   fn on_transfer_event_received (
           &mut self,
           slot: &mut super::device_slot::DeviceSlot<Doorbell, Memory>,
           transfer_event: xhci::ring::trb::event::TransferEvent,
           target_event: crate::xhc::transfer::event::target_event::TargetEvent,
       ) -> (super::phase::InitStatus, Option<alloc::boxed::Box<dyn Phase<Doorbell, Memory>>>) {
       let data_stage = target_event.data_stage();
       let conf_desc_buff = data_stage.data_buffer_pointer() as *mut u8;
       let conf_desc_buff_len = (data_stage.trb_transfer_length() - transfer_event.trb_transfer_length()) as usize;

       let conf_desc = unsafe {*(data_stage.data_buffer_pointer() as *const ConfigurationDescriptor)};
       let descriptors = DescriptorSequence::new(conf_desc_buff, conf_desc_buff_len).collect::<Vec<Descriptor>>();

       let hid_device_descriptors: Vec<HidDeviceDescriptors> = descriptors
            .iter()
            .enumerate()
            .filter_map(filter_interface)
            .filter(|(index, interface)| filter_mouse_or_keyboard((*index, interface)))
            .filter_map(|(index, interface)| map_hid_descriptors(index, interface, &descriptors))
            .collect();

        slot.input_context_mut()
            .set_config(conf_desc.configuration_value);

        set_configuration(
            conf_desc.configuration_value as u16,
            slot.default_control_pipe_mut(),
        );

        (
            InitStatus::not(),
            Some(Box::new(Phase3::new(
                self.mouse.clone(),
                hid_device_descriptors,
            ))),
        )
   } 

fn interface_nums(&self) -> Option<Vec<u8>> {
       None
   }
}

fn set_configuration<T: DoorbellExternalRegisters>(
    config_value: u16,
    default_control_pipe: &mut ControlPipe<T>,
) {
    default_control_pipe
        .control_out()
        .no_data(Request::configuration(config_value))
}


fn filter_interface((index, device): (usize, &Descriptor)) -> Option<(usize, InterfaceDescriptor)> {
    device
        .interface()
        .map(|interface| (index, interface.clone()))
}


fn filter_mouse_or_keyboard((_, interface): (usize, &InterfaceDescriptor)) -> bool {
    interface.is_mouse()
}


fn map_hid_descriptors(
    index: usize,
    interface: InterfaceDescriptor,
    descriptors: &[Descriptor],
) -> Option<HidDeviceDescriptors> {
    let endpoint = descriptors
        .iter()
        .skip(index + 1 + 1)
        .find_map(|descriptor| {
            if let Descriptor::Endpoint(endpoint) = descriptor {
                Some(endpoint)
            } else {
                None
            }
        })?;
    Some(HidDeviceDescriptors::new(interface, endpoint.clone()))
}