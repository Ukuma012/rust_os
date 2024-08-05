#![no_std]
#![no_main]
#![feature(vec_into_raw_parts)]

extern crate alloc;

use core::{slice, mem, arch::asm};
use alloc::vec;
use alloc::vec::Vec;
use uefi::{
    prelude::*,
    proto::{
        console::gop::{GraphicsOutput, PixelFormat},
        loaded_image::LoadedImage,
        media::{
            file::{Directory, File, FileAttribute, FileInfo, FileMode, RegularFile},
            fs::SimpleFileSystem,
        },
    },
    table::{
        boot::{self, MemoryType},
        cfg::ACPI_GUID,
    },
    CStr16,
};
use uefi::table::Runtime;
use log::{trace, info};
use goblin::elf::{Elf, program_header};
use uefi::table::boot::MemoryDescriptor;
use common::frame_buffer;
use common::memory_map;

const UEFI_PAGE_SIZE: usize = 0x1000;

#[entry]
fn efi_main(handle: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut st).unwrap();

    st.stdout().reset(false).unwrap();
    let bs = st.boot_services();

    trace!("get_memory_map");
    get_memory_map(bs);

    trace!("load kernel");
    let elf_entry = load_kernel(handle, bs);

    trace!("entry_point_addr = 0x{:x}", elf_entry);
    let entry_point: extern "sysv64" fn(&frame_buffer::FrameBufferConfig, &memory_map::MemoryMap, u64) = unsafe {
        mem::transmute(elf_entry)
    };

    info!("get_frame_buffer_config");
    let frame_buffer = get_frame_buffer(st.boot_services());

    trace!("get_rsdp");
    let rsdp = get_rsdp(&st);

    trace!("exit_boot_serveces");
    let (_st, memory_map) = exit_boot_services(handle, st);

    entry_point(&frame_buffer, &memory_map, rsdp);

    trace!("you cannot see this message");


    loop {
        unsafe {asm!("hlt")}
    }
}

fn get_rsdp(st: &SystemTable<Boot>) -> u64 {
    st.config_table().iter().find(|config| config.guid == ACPI_GUID).map(|config| config.address as u64).expect("Could not find RSDP")
}

fn get_memory_map(boot_services: &BootServices) {
    let map_size = boot_services.memory_map_size().map_size;
    let mut memmap_buf = vec![0; map_size * 16];
    let (_map_key, desc_itr) = boot_services.memory_map(&mut memmap_buf).unwrap();
    let _descriptors = desc_itr.copied().collect::<Vec<_>>();
    // descriptors.iter().for_each(|descriptor| {
    //     info!("{:?}, {}, {}, {}", descriptor.ty, descriptor.phys_start, descriptor.virt_start, descriptor.page_count);
    // })
}

fn open_directory(_image: Handle, boot_services: &BootServices) -> Directory {
    let loaded_image = boot_services.handle_protocol::<LoadedImage>(_image).unwrap().get();
    let device = unsafe{(*loaded_image).device()};
    let file_system = boot_services.handle_protocol::<SimpleFileSystem>(device).unwrap().get();
    unsafe {
        (*file_system).open_volume().unwrap()
    }
}

fn open_file(root_dir: &mut Directory) -> RegularFile {
    let mut cstr_buf = [0u16; 32];
    let cstr_file_name = CStr16::from_str_with_buf("kernel.elf", &mut cstr_buf).unwrap();
    let file_handle = root_dir.open(cstr_file_name, FileMode::Read, FileAttribute::empty()).unwrap();
    unsafe {
        RegularFile::new(file_handle)
    }
}

fn load_elf(boot_services: &BootServices, buf: Vec<u8>) -> usize {
    let elf = Elf::parse(&buf).unwrap();

    let mut dest_start = usize::MAX;
    let mut dest_end = 0;
    for ph in elf.program_headers.iter() {
        info!("Program header: {} {} {} {} {}", program_header::pt_to_str(ph.p_type), ph.p_offset, ph.p_vaddr, ph.p_paddr, ph.p_memsz);
        if ph.p_type != program_header::PT_LOAD {
            continue;
        }
        dest_start = dest_start.min(ph.p_paddr as usize);
        dest_end = dest_end.max(ph.p_paddr + ph.p_memsz);
    }

    boot_services.allocate_pages(boot::AllocateType::Address(dest_start), MemoryType::LOADER_DATA, (dest_end as usize - dest_start as usize + UEFI_PAGE_SIZE - 1) / UEFI_PAGE_SIZE).unwrap();

    for ph in elf.program_headers.iter() {
        if ph.p_type != program_header::PT_LOAD {
            continue;
        }
        let dest = unsafe {
            slice::from_raw_parts_mut(ph.p_paddr as *mut u8, ph.p_memsz as usize)
        };
        dest[..(ph.p_filesz as usize)].copy_from_slice(&buf[(ph.p_offset as usize)..(ph.p_offset as usize + ph.p_filesz as usize)]);
        dest[(ph.p_filesz as usize)..].fill(0);
    };
    
    elf.entry as usize
} 

fn load_kernel(_image: Handle, boot_services: &BootServices) -> usize {
    let mut root_dir = open_directory(_image, &boot_services);
    let mut file = open_file(&mut root_dir);
    let file_size = file.get_boxed_info::<FileInfo>().unwrap().file_size() as usize;
    let mut buf = vec![0; file_size];
    let _ = file.read(&mut buf);
    file.close();
    load_elf(&boot_services, buf)
}

fn get_frame_buffer(boot_services: &BootServices) -> frame_buffer::FrameBufferConfig {
    let gop = boot_services.locate_protocol::<GraphicsOutput>().unwrap();
    let gop = unsafe {&mut *gop.get()};
    frame_buffer::FrameBufferConfig {
        frame_buffer: gop.frame_buffer().as_mut_ptr(),
        stride: gop.current_mode_info().stride() as u32,
        resolution: (
            gop.current_mode_info().resolution().0 as u32,
            gop.current_mode_info().resolution().1 as u32,
        ),
        format: match gop.current_mode_info().pixel_format() {
            PixelFormat::Rgb => frame_buffer::PixelFormat::Rgb,
            PixelFormat::Bgr => frame_buffer::PixelFormat::Bgr,
            f => panic!("Unsupported pixel format: {:?}", f),
        },
    }
}

fn exit_boot_services(
    image: Handle,
    st: SystemTable<Boot>,
) -> (SystemTable<Runtime>, memory_map::MemoryMap) {
    let enough_mmap_size =
        st.boot_services().memory_map_size().map_size + 8 * mem::size_of::<MemoryDescriptor>();
    let mmap_buf = vec![0; enough_mmap_size].leak();
    let mut descriptors = Vec::with_capacity(enough_mmap_size);
    let (st, raw_descriptors) = st
        .exit_boot_services(image, mmap_buf)
        .expect("Failed to exit boot services");

    // uefi::MemoryDescriptor -> memory_map::Descriptor
    for d in raw_descriptors {
        if is_available_after_exit_boot_services(d.ty) {
            descriptors.push(memory_map::Descriptor {
                memory_type: d.ty.0,
                phys_start: d.phys_start,
                phys_end: d.phys_start + d.page_count * UEFI_PAGE_SIZE as u64,
                virt_start: d.virt_start,
                att: d.att.bits(),
            });
        }
    }
    let memory_map = {
        let (ptr, len, _) = descriptors.into_raw_parts();
        memory_map::MemoryMap {
            descriptors: ptr as *const memory_map::Descriptor,
            descriptors_len: len as u64,
        }
    };
    (st, memory_map)
}

fn is_available_after_exit_boot_services(ty: MemoryType) -> bool {
    matches!(
        ty,
        MemoryType::CONVENTIONAL | MemoryType::BOOT_SERVICES_CODE | MemoryType::BOOT_SERVICES_DATA
    )
}