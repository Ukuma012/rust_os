pub struct AlignedAddress(u64);

impl AlignedAddress {
    pub fn new_uncheck(addr: u64) -> Self {
        Self(addr)
    }

    pub fn new_with_check_align_64_bytes(addr: u64) -> Self {
        if is_align_64_bytes(addr) {
            Self::new_uncheck(addr)
        } else {
            panic!("Not aligned address address = {addr} expect align size = {}", 64);
        }
    }

    pub fn address(&self) -> u64 {
        if is_align_64_bytes(self.0) {
            self.0
        } else {
            panic!("Not aligned address address = {} expect align size = {}", self.0, 64)
        }
    }
}

fn is_align_64_bytes(value: u64) -> bool {
    (value % 64) == 0
}