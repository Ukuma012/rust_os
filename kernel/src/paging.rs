// 4levelではなく、3levelになっている

use x86_64::{registers::control::{Cr3, Cr3Flags}, structures::paging::{PageSize, PageTable, PhysFrame, Size1GiB, Size2MiB}, PhysAddr};
use spin::Lazy;

const EMPTY_PAGE_TABLE: PageTable = PageTable::new();

static PAGE_TABLE: Lazy<PhysFrame> = Lazy::new(|| unsafe { init_identity_page_table() }); 
static mut PML4_TABLE: PageTable = PageTable::new(); // Page Map Level4 Table
static mut PDP_TABLE: PageTable = PageTable::new();  // Page Directory Pointer Table
static mut PAGE_DIRECTORY: [PageTable; 64] = [EMPTY_PAGE_TABLE; 64];

pub unsafe fn init() {
    Cr3::write(*PAGE_TABLE, Cr3Flags::empty());
}

unsafe fn init_identity_page_table() -> PhysFrame {
    use x86_64::structures::paging::PageTableFlags;

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::GLOBAL;

    // PML4_TABLE[0] -> PDP_TABLE
    PML4_TABLE[0].set_frame(phys_frame(&PDP_TABLE), flags);

    for(i, d) in PAGE_DIRECTORY.iter_mut().enumerate() {
        // PDP_TABLE[i] -> PAGE_DIRECTORY[i]
        PDP_TABLE[i].set_frame(phys_frame(d), flags);

        for(j, p) in PAGE_DIRECTORY[i].iter_mut().enumerate() {
            // PAGE_DIRECTORY[i][j] -> (identical mapping)
            let addr = PhysAddr::new(i as u64 * Size1GiB::SIZE + j as u64 * Size2MiB::SIZE);
            p.set_addr(addr, flags | PageTableFlags::HUGE_PAGE);
        }
    }

    phys_frame(&PML4_TABLE)
}

unsafe fn phys_frame(page_table: &'static PageTable) -> PhysFrame {
    PhysFrame::from_start_address(
        PhysAddr::new(page_table as *const PageTable as u64)
    ).unwrap()
}