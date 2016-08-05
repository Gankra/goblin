pub use super::super::program_header::*;

#[repr(C)]
#[derive(Clone, PartialEq, Default)]
#[derive(Debug)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}

pub const SIZEOF_PHDR: usize = 32;
