#[derive(Debug, Clone)]
pub struct MessageAddressAccessor {
    is_64bit_address: bool,
}

impl MessageAddressAccessor {
    pub fn new(is_64bit_address: bool) -> Self {
        Self {
            is_64bit_address
        }
    }
}