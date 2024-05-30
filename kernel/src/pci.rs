#[no_std]
#[no_main]
use lazy_static::lazy_static;
use spin::Mutex;

const CONFIG_ADDRESS: u16 = 0x0cf8;
const CONFIG_DATA: u16 = 0x0cfc;


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