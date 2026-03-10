// memory/map.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryRegionType {
    Usable,
    Reserved,
    Acpi,
}

#[derive(Clone, Copy, Debug)]
pub struct MemoryRegion {
    pub start: usize,
    pub end: usize,
    pub region_type: MemoryRegionType,
}

#[repr(C, packed)]
pub struct MultibootTag {
    pub typ: u32,
    pub size: u32,
}

#[repr(C, packed)]
pub struct MultibootMemoryMapTag {
    pub typ: u32,
    pub size: u32,
    pub entry_size: u32,
    pub entry_version: u32,
}

#[repr(C, packed)]
pub struct MultibootMemoryMapEntry {
    pub base_addr: u64,
    pub length: u64,
    pub typ: u32,
    pub reserved: u32,
}

pub struct MemoryMap {
    pub regions: &'static [MemoryRegion],
}

pub unsafe fn from_multiboot(multiboot_addr: usize) -> MemoryMap {
    static mut REGIONS: [MemoryRegion; 128] = [MemoryRegion {
        start: 0,
        end: 0,
        region_type: MemoryRegionType::Reserved,
    }; 128];

    let mut count = 0;

    parse_memory_map(multiboot_addr, |region| {
        REGIONS[count] = region;
        count += 1;
    });

    MemoryMap {
        regions: &REGIONS[..count],
    }
}

const TAG_MEMORY_MAP: u32 = 6;

pub unsafe fn parse_memory_map(
    multiboot_addr: usize,
    mut callback: impl FnMut(MemoryRegion),
) {
    let mut tag = (multiboot_addr + 8) as *const MultibootTag;

    loop {
        match (*tag).typ {
            TAG_MEMORY_MAP => {
                let mmap = &*(tag as *const MultibootMemoryMapTag);

                let entries_base = (mmap as *const _ as usize
                    + core::mem::size_of::<MultibootMemoryMapTag>())
                    as *const MultibootMemoryMapEntry;

                let entry_count =
                    (mmap.size as usize - core::mem::size_of::<MultibootMemoryMapTag>())
                        / mmap.entry_size as usize;

                for i in 0..entry_count {
                    let entry = &*entries_base.add(i);

                    let mut start = entry.base_addr;
                    let mut length = entry.length;

                    // Ignora regiões abaixo de 1 MiB
                    if start + length <= 0x100000 {
                        continue;
                    }

                    if start < 0x100000 {
                        let diff = 0x100000 - start;
                        start += diff;
                        length -= diff;
                    }

                    let region_type = match entry.typ {
                        1 => MemoryRegionType::Usable,
                        3 => MemoryRegionType::Acpi,
                        _ => MemoryRegionType::Reserved,
                    };

                    callback(MemoryRegion {
                        start: start as usize,
                        end: (start + length) as usize,
                        region_type,
                    });
                }

                return;
            }
            0 => break,
            _ => {}
        }

        tag = ((tag as usize + (*tag).size as usize + 7) & !7)
            as *const MultibootTag;
    }

    panic!("Multiboot memory map not found");
}
