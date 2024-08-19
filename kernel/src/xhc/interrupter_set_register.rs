use super::external_reg::ExternalRegisters;

pub trait InterrupterSetRegisterOperations {
    fn clear_interrupt_pending_at(&mut self, index: usize);

    fn clear_event_handler_busy_at(&mut self, index: usize);

    fn set_counter_at(&mut self, index: usize, count: u16);

    fn write_event_ring_dequeue_pointer_at(
        &mut self,
        index: usize,
        event_ring_segment_addr: u64,
    );

    fn write_event_ring_segment_table_pointer_at(
        &mut self,
        index: usize,
        event_ring_segment_table_addr: u64,
    );

    fn write_interrupter_enable_at(&mut self, index: usize, is_enable: bool);

    fn write_interrupter_pending_at(&mut self, index: usize, is_pending: bool);

    fn read_dequeue_pointer_addr_at(&mut self, index: usize) -> u64;

    fn write_event_ring_segment_table_size(&mut self, index: usize, size: u16);

    fn update_dequeue_pointer_at(&mut self, index: usize, dequeue_pointer_addr: u64) {
        self.write_interrupter_pending_at(index, true);
        self.write_event_ring_dequeue_pointer_at(index, dequeue_pointer_addr);
    }
}

impl<M> InterrupterSetRegisterOperations for ExternalRegisters<M>
where 
    M: xhci::accessor::Mapper + Clone,
{
    fn write_event_ring_dequeue_pointer_at(
            &mut self,
            index: usize,
            event_ring_segment_addr: u64,
        ) {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .erdp
            .update_volatile(|erdp| erdp.set_event_ring_dequeue_pointer(event_ring_segment_addr));

        self.clear_event_handler_busy_at(index);
    }

    fn clear_event_handler_busy_at(&mut self, index: usize) {
        self.0
            .interrupter_register_set
            .interrupter_mut(index)
            .erdp
            .update_volatile(|erdp| {
                erdp.clear_event_handler_busy();
            });
    }

    fn write_event_ring_segment_table_pointer_at(
            &mut self,
            index: usize,
            event_ring_segment_table_addr: u64,
        ) {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .erstba
            .update_volatile(|erstba| erstba.set(event_ring_segment_table_addr))
    }

    fn write_interrupter_enable_at(&mut self, index: usize, is_enable: bool) {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .iman
            .update_volatile(|iman| {
                if is_enable {
                    iman.set_interrupt_enable();
                } else {
                    iman.clear_interrupt_enable();
                }
            });
    }

    fn write_interrupter_pending_at(&mut self, index: usize, is_pending: bool) {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .iman
            .update_volatile(|iman| {
                if is_pending {
                    iman.set_0_interrupt_pending();
                } else {
                    iman.clear_interrupt_pending();
                }
            });
    }

    fn read_dequeue_pointer_addr_at(&mut self, index: usize) -> u64 {
        self.0
            .interrupter_register_set
            .interrupter(index)
            .erdp
            .read_volatile()
            .event_ring_dequeue_pointer()
    }

    fn write_event_ring_segment_table_size(&mut self, index: usize, size: u16) {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .erstsz
            .update_volatile(|e| e.set(size));
    }

    fn set_counter_at(&mut self, index: usize, count: u16) {
        self.0
            .interrupter_register_set
            .interrupter_mut(index)
            .imod
            .update_volatile(|imod| {
                imod.set_interrupt_moderation_counter(count);
            });
    }

    fn clear_interrupt_pending_at(&mut self, index: usize) {
        self.0
            .interrupter_register_set
            .interrupter_mut(index)
            .iman
            .update_volatile(|iman| {
                iman.clear_interrupt_pending();
            });
    }


}