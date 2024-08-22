pub mod mouse;
mod boot_protocol_buffer;
pub mod interrupt_in;

pub trait ClassDriverOperate {
    fn on_data_received(&mut self);

    fn data_buff_addr(&self) -> u64;

    fn data_buff_len(&self) -> u32;
}