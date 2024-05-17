#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::asm, slice};

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum BootPixelFormat {
    Rgb,
    Bgr,
}

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct FrameBuffer {
    pub frame_buffer: *mut u8,
    pub stride: u32,
    pub resolution: (u32, u32), // (horizontal, vertical)
    pub format: BootPixelFormat,
}

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct MemoryMap {
    pub descriptors: *const Descriptor,
    pub descriptors_len: u64,
}

impl MemoryMap {
    pub fn descriptors(&self) -> &[Descriptor] {
        unsafe { slice::from_raw_parts(self.descriptors, self.descriptors_len as usize)}
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Descriptor {
    pub phys_start: u64,
    pub phys_end: u64,
}

struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
}

const kFontA: [u8; 16] = [
    0b00000000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00100100,
    0b00100100,
    0b00100100,
    0b00100100,
    0b01111110,
    0b01000010,
    0b01000010,
    0b01000010,
    0b11100111,
    0b00000000,
    0b00000000,
];

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer: &FrameBuffer, _memory_map: &MemoryMap) {

    let white = PixelColor {
        r: 255,
        g: 255,
        b: 255,
    };

    let green = PixelColor {
        r: 51,
        g: 255,
        b: 51
    };

    let black = PixelColor {
        r: 0,
        g: 0,
        b: 0 
    };

    for x in 0..frame_buffer.resolution.0 {
        for y in 0..frame_buffer.resolution.1 {
            write_pixel(frame_buffer, x, y, &white);
        }
    }

    for x in 0..200 {
        for y in 0..100 {
            write_pixel(frame_buffer, 100+x, 100+y, &green);
        }
    }

    write_ascii(frame_buffer, 100, 50, 'A', &black);
    write_ascii(frame_buffer, 108, 50, 'A', &black);


    loop {
        unsafe {asm!("hlt")}
    }
}

fn write_ascii(config: &FrameBuffer, x: u32, y: u32, c: char, color: &PixelColor) {
    if c != 'A' {
        return;
    }

    for dy in 0..16 {
        for dx in 0..8 {
            if (kFontA[dy] << dx) & 0x80u8 != 0 {
                write_pixel(config, x+dx, y+dy as u32, color);
            }
        }
    }
}

fn write_pixel(config: &FrameBuffer, x: u32, y: u32, c: &PixelColor) {
    let pixel_position = config.stride * y + x;
    let base: isize = (4 * pixel_position) as isize;

    unsafe {
        let p = config.frame_buffer.offset(base);
            *p.offset(0) = c.r;
            *p.offset(1) = c.g;
            *p.offset(2) = c.b;
    }

    // if config.format == BootPixelFormat::Rgb {
    //     unsafe {
    //         let p = config.frame_buffer.offset(base);
    //         *p.offset(0) = c.r;
    //         *p.offset(1) = c.g;
    //         *p.offset(2) = c.b;
    //     }
    // } else if config.format == BootPixelFormat::Bgr {
    //     unsafe {
    //         let p = config.frame_buffer.offset(base);
    //         *p.offset(0) = c.r;
    //         *p.offset(1) = c.g;
    //         *p.offset(2) = c.b;
    //     }
    // }
} 


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {asm!("hlt")}
    }
}