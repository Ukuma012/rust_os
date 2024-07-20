use core::mem;
use lazy_static::lazy_static;
use common::memory_map::MemoryMap;
use log::trace;
use x86_64::PhysAddr;
use spin::mutex::Mutex;

// lazy_static! {
//     static ref FRAME_MANAGER: Mutex<BitmapMemoryManager> = Mutex::new(BitmapMemoryManager::new());
// }

// pub fn frame_manager() -> spin::MutexGuard<'static, BitmapMemoryManager> {
//     FRAME_MANAGER.lock()
// }

static mut FRAME_MANAGER: Option<Mutex<BitmapMemoryManager>> = None;

pub fn frame_manager() -> &'static Mutex<BitmapMemoryManager> {
    unsafe {
        // 初めてアクセスされたときに初期化を行います。
        if FRAME_MANAGER.is_none() {
            FRAME_MANAGER = Some(Mutex::new(BitmapMemoryManager::new()));
        }
        FRAME_MANAGER.as_ref().unwrap()
    }
}

pub fn lock_frame_manager() -> spin::MutexGuard<'static, BitmapMemoryManager> {
    frame_manager().lock()
}

const MAX_PHYSICAL_MEMORY_BYTES: usize = 128 * 1024 * 1024 * 1024; // 128GiB
const FRAME_COUNT: usize = MAX_PHYSICAL_MEMORY_BYTES / Frame::SIZE; // address0 ~ 4095 -> Frame0, addr4096 ~ 8192 -> Frame1
type MapLine = usize;
const BITS_PER_MAP_LINE: usize = 8 * mem::size_of::<MapLine>();
const MAP_LINE_COUNT: usize = FRAME_COUNT / BITS_PER_MAP_LINE;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Frame(usize);

impl Frame {
    pub const SIZE: usize = 4096; // 4KiB
    const MIN: Self = Self(1);
    const MAX: Self = Self(FRAME_COUNT);

    fn offset(self, offset: usize) -> Self {
        Self(self.0 + offset)
    }

    fn phys_addr(self) -> PhysAddr {
        PhysAddr::new((self.0 * Frame::SIZE) as u64)
    }

    fn from_phys_addr(addr: PhysAddr) -> Self {
        Self(addr.as_u64() as usize / Frame::SIZE)
    }
}

pub struct BitmapMemoryManager {
    alloc_map: [MapLine; MAP_LINE_COUNT],
    begin: Frame,
    end: Frame,
}

impl BitmapMemoryManager {
    pub fn new() -> Self {
        Self {
            alloc_map: [0; MAP_LINE_COUNT],
            begin: Frame::MIN,
            end: Frame::MAX,
        }
    }

    fn set_bit(&mut self, frame: Frame, allocated: bool) {
        let line_index = frame.0 / BITS_PER_MAP_LINE;
        let bit_index = frame.0 % BITS_PER_MAP_LINE;

        if allocated {
            self.alloc_map[line_index] |= 1 << bit_index;
        } else {
            self.alloc_map[line_index] &= !(1 << bit_index);
        }
    }

    fn get_bit(&self, frame: Frame) -> bool {
        let line_index = frame.0 / BITS_PER_MAP_LINE;
        let bit_index = frame.0 % BITS_PER_MAP_LINE;
        (self.alloc_map[line_index] & (1 << bit_index)) != 0
    }

    fn set_memory_range(&mut self, begin: Frame, end: Frame) {
        self.begin = begin;
        self.end = end;
    }

    fn mark_allocated_in_bytes(&mut self, start: Frame, bytes: usize) {
        self.mark_allocated(start, bytes / Frame::SIZE, true)
    }

    fn mark_allocated(&mut self, frame: Frame, num_frames: usize, init: bool) {
        for i in 0..num_frames {
            if !init {
                trace!("phys_memory: allocate {:?}", frame.offset(i).phys_addr());
            }
            self.set_bit(frame.offset(i), true);
        }
    }

    pub fn init(&mut self, memory_map: &MemoryMap) {
        let mut phys_available_end = 0;
        for d in memory_map.descriptors() {
            let phys_start = d.phys_start as usize;
            let phys_end = d.phys_end as usize;
            if phys_available_end < d.phys_start as usize {
                self.mark_allocated_in_bytes(Frame::from_phys_addr(PhysAddr::new(phys_available_end as u64)), phys_start - phys_available_end);
            }
            phys_available_end = phys_end;
        }
        self.set_memory_range(Frame::MIN, Frame::from_phys_addr(PhysAddr::new(phys_available_end as u64)));
    }

    pub fn alocate(&mut self, num_frames: usize) -> Result<Frame, AllocateError> {
        let mut frame = self.begin;
        loop {
            for i in 0..num_frames {
                if frame.offset(i) >= self.end {
                    Err(AllocateError::NotEnoughFrame)?
                }
                if self.get_bit(frame.offset(i)) {
                    frame = frame.offset(i + 1);
                    continue;
                }
            }
            self.mark_allocated(frame, num_frames, false);
            return Ok(frame);
        }
    }

    pub fn free(&mut self, frame: Frame, num_frames: usize) {
        for i in 0..num_frames {
            trace!("phys_memory: deallocate {:?}", frame.offset(i).phys_addr());
            self.set_bit(frame.offset(i), false);
        }
    }
    
}

pub enum AllocateError {
    NotEnoughFrame,
}