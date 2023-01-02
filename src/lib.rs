#![feature(get_mut_unchecked)] // We use this to mutate class data while having references to the class in other objects.
#![feature(map_try_insert)]
// We use this feature to avoid updating field entry keys. 
// We could also check beforehand, but this works simpler (even though it's experimental).
#![feature(let_chains)] // We use this to chain let comparisons, which makes our code look cleaner
#![feature(downcast_unchecked)] // Used for downcasting to native objects.
#![feature(allocator_api, ptr_internals)]
#![feature(vec_into_raw_parts)]
#![feature(int_roundings)]
#![feature(float_next_up_down)]

#[macro_use] pub mod access_macros;
pub mod argsparser;
pub mod jvm;
pub mod class;
pub mod data_access;
pub mod errorcodes;
pub mod thread;
pub mod frame;
pub mod value;
pub mod attributes;
pub mod constant_pool;
pub mod flags;
pub mod reference;
pub mod multitypebox;
pub mod llvm;

#[cfg(test)]
pub mod testing;

use std::{fs::File, fs, io::Read};
use {class::classfile::ClassFile};

pub fn load_class(f: &mut File, path: &str) -> (ClassFile, Vec<Vec<u8>>) {
    unsafe {
        let metadata = fs::metadata(path).unwrap();
        let size = if metadata.len() % 8 == 0 {metadata.len() / 8} else {metadata.len() / 8 + 1};
        let buffer: Vec<u64> = vec![0; size as usize];
        let buf_bytes = std::slice::from_raw_parts_mut(buffer.as_ptr() as *mut u8, buffer.len() * std::mem::size_of::<u64>());
        f.read(buf_bytes).unwrap();
        ClassFile::new(buf_bytes).unwrap()
    }
}