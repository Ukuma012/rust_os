#[derive(Debug)]
#[repr(transparent)]
pub struct BootProtocolBuffer<'buff>(&'buff[i8]);

impl<'buff> BootProtocolBuffer<'buff> {
    pub fn new(data_buff: &'buff [i8]) -> Self {
        Self(data_buff)
    }

    pub fn buff(&self) -> &[i8] {
        self.0
    }
}