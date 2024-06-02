use crate::error::{Code, Error};
use crate::make_error;
use lazy_static::lazy_static;
use spin::mutex::Mutex;

const CONFIG_ADDRESS: u16 = 0x0cf8;
const CONFIG_DATA: u16 = 0x0cfc;
const NON_EXISTENT_DEVICE: u16 = 0xffff;

lazy_static! {
   static ref NUM_DEVICE: Mutex<usize> = Mutex::new(0); 
}

#[derive(Copy, Clone, Debug)]
pub struct Device {
    bus: u8,
    device: u8,
    function: u8,
    header_type: u8,
    class_code: ClassCode,
}

impl Device {
    fn new(bus: u8, device: u8, function: u8, header_type: u8, class_code: ClassCode) -> Self {
        Self {
            bus,
            device,
            function,
            header_type,
            class_code,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct ClassCode {
    base: u8,
    sub: u8,
    interface: u8,
}

impl ClassCode {
    fn new(base: u8, sub: u8, interface: u8) -> Self {
        Self {
            base,
            sub,
            interface,
        }
    }

    fn is_match_base(&self, base: u8) -> bool {
        base == self.base
    }

    fn is_match_base_sub(&self, base: u8, sub: u8) -> bool {
        self.is_match_base(base) && sub == self.sub
    }
}

impl From<u32> for ClassCode {
    fn from(reg: u32) -> Self {
        let base = (reg >> 24) as u8;
        let sub = (reg >> 16) as u8;
        let interface = (reg >> 8) as u8;
        ClassCode::new(base, sub, interface)
    }
}

lazy_static! {
    static ref DEVICES: Mutex<[Device; 32]> = Mutex::new([Device {
        bus: 0,
        device: 0,
        function: 0,
        header_type: 0,
        class_code: ClassCode { base: 0, sub: 0, interface: 0 },
    }; 32]);
}

fn make_address(bus: u8, device: u8, function: u8, reg_addr: u8) -> u32 {
    fn shl(x: u8, bits: usize) -> u32 {
        let x = x as u32;
        x << bits
    }

    let enabled_bit: u32 = (1 as u32) << 31;
    enabled_bit | shl(bus, 16) | shl(device, 11) | shl(function, 8) | (reg_addr & 0xfc) as u32
}

extern "C" {
    fn IoOut32(addr: u16, data: u32);
    fn IoIn32(addr: u16) -> u32;
}

fn write_address(address: u32) {
    unsafe { IoOut32(CONFIG_ADDRESS, address) }
}

fn write_data(value: u32) {
    unsafe { IoOut32(CONFIG_DATA, value) }
}

fn read_data() -> u32 {
    unsafe { IoIn32(CONFIG_DATA)}
}

fn read_vendor_id(bus: u8, device: u8, function: u8) -> u16 {
    write_address(make_address(bus, device, function, 0x00));
    read_data() as u16
}

fn read_device_id(bus: u8, device: u8, function: u8) -> u16 {
    write_address(make_address(bus, device, function, 0x00));
    (read_data() >> 16) as u16
}

fn read_header_type(bus: u8, device: u8, function: u8) -> u8 {
    write_address(make_address(bus, device, function, 0x0c));
    (read_data() >> 16) as u8
}

fn read_class_code(bus: u8, device: u8, function: u8) -> ClassCode {
    write_address(make_address(bus, device, function, 0x08));
    let reg = read_data();
    ClassCode::from(reg)
}

fn read_bus_number(bus: u8, device: u8, function: u8) -> u32 {
    write_address(make_address(bus, device, function, 0x18));
    read_data()
}

fn read_conf_reg(device: &Device, reg_addr: u8) -> u32 {
    let address = make_address(device.bus, device.device, device.function, reg_addr);
    write_address(address);
    read_data()
}

fn write_conf_reg(device: &Device, reg_addr: u8, value: u32) {
    let address = make_address(device.bus, device.device, device.function, reg_addr);
    write_address(address);
    write_data(value);
}

pub fn scan_all_bus() -> Result<(), Error> {
    let header_type = read_header_type(0, 0, 0);
    if is_single_function_device(header_type) {
        return scan_bus(0);
    }

    for function in 1..8 as u8 {
        if read_vendor_id(0, 0, function) == NON_EXISTENT_DEVICE {
            continue;
        }

        let bus = function;
        scan_bus(bus)?;
    }

    Ok(())
}

fn scan_bus(bus: u8) -> Result<(), Error> {
    for device in 0..32 as u8 {
        if read_vendor_id(bus, device, 0) == NON_EXISTENT_DEVICE {
            continue;
        }
        scan_device(bus, device)?;
    }

    Ok(())
}

fn scan_device(bus: u8, device: u8) -> Result<(), Error> {
    scan_function(bus, device, 0)?;

    if is_single_function_device(read_header_type(bus, device, 0)) {
        return Ok(());
    }

    for function in 1..8 as u8 {
        if read_vendor_id(bus, device, function) == NON_EXISTENT_DEVICE {
            continue;
        }
        scan_function(bus, device, function)?;
    }

    Ok(())
}

fn scan_function(bus: u8, device: u8, function: u8) -> Result<(), Error> {
    let class_code = read_class_code(bus, device, function);
    let header_type = read_header_type(bus, device, function);
    add_device(bus, device, function, header_type, class_code)?;

    // if the device is a PCI to PCI bridge
    if class_code.is_match_base_sub(0x06, 0x04) {
        // scan pci devices whtich are connected with the secondary_bus
        let bus_numbers = read_bus_number(bus, device, function);
        let secondary_bus = (bus_numbers >> 8) & 0xff;
        return scan_bus(secondary_bus as u8);
    }

    Ok(())
}

fn add_device(bus: u8, device: u8, function: u8, header_type: u8, class_code: ClassCode) -> Result<(), Error> {
    let mut num_device = NUM_DEVICE.lock();
    if *num_device == DEVICES.lock().len() {
        return Err(make_error!(Code::Full))
    }

    let mut devices = DEVICES.lock();
    devices[*num_device] = Device::new(bus, device, function, header_type, class_code);
    *num_device += 1;

    Ok(())
}

fn is_single_function_device(header_type: u8) -> bool {
    header_type & 0x80 == 0
}