use alloc::vec::Vec;
use super::external_reg::ExternalRegisters;

pub trait PortExternalRegisterss {
    fn reset_port_at(&mut self, port_id: u8);
    fn read_port_spped_at(&self, port_id: u8);
    fn read_port_reset_change_status(&self, port_id: u8);
    fn clear_port_reset_change_at(&mut self, port_id: u8);
    fn reset_all(&mut self);
    fn connectiong_ports(&self) -> Vec<u8>;
}

impl<M> PortExternalRegisterss for ExternalRegisters<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn reset_port_at(&mut self, port_id: u8) {
        self.registers_mut()
            .port_register_set
            .update_volatile_at(port_id as usize, |port| {
                port.portsc.set_port_reset();
            });

        while self
            .0
            .port_register_set
            .read_volatile_at(port_id as usize)
            .portsc
            .port_reset()
            {}
    }

    fn read_port_spped_at(&self, port_id: u8) {
        self.0 
            .port_register_set
            .read_volatile_at(port_index(port_id))
            .portsc
            .port_speed();
    }

    fn read_port_reset_change_status(&self, port_id: u8) {
        self.0
            .port_register_set
            .read_volatile_at(port_index(port_id))
            .portsc
            .port_reset_change();
    }

    fn clear_port_reset_change_at(&mut self, port_id: u8) {
        self.registers_mut()
            .port_register_set
            .update_volatile_at(port_index(port_id), |port| {
                port.portsc
                    .set_0_port_reset_change();
            });
    }

    fn reset_all(&mut self) {
        let ports = self
            .0
            .port_register_set
            .into_iter()
            .enumerate()
            .filter(|(_, p)| {
                p.portsc
                    .current_connect_status()
            })
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();


        self.0
            .port_register_set
            .update_volatile_at(ports[0], |p| {
                p.portsc.set_port_reset();
            });

        while self
            .0
            .port_register_set
            .read_volatile_at(ports[0])
            .portsc
            .port_reset()
        {}
    }

    fn connectiong_ports(&self) -> Vec<u8> {
        self.0
            .port_register_set
            .into_iter()
            .enumerate()
            .filter(|(_, p)| {
                p.portsc
                    .current_connect_status()   
            })
            .map(|(id, _)| id as u8)
            .collect()
    }
}

fn port_index(port_id: u8) -> usize {
    (port_id - 1) as usize
}