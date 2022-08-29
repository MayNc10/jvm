#![feature(get_mut_unchecked)] // We use this to mutate class data while having references to the class in other objects.
#![feature(map_try_insert)]
// We use this feature to avoid updating field entry keys. 
// We could also check beforehand, but this works simpler (even though it's experimental).

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