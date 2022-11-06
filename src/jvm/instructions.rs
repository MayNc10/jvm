use super::JVM;

use crate::access_macros;
use crate::class::Class;
use crate::constant_pool::Entry;
use crate::errorcodes::{Error, Opcode};
use crate::reference::{Reference, Monitor};
use crate::value::{Value, VarValue};

use std::collections::HashMap;
use std::rc::Rc;

pub mod constants;
pub mod loads;
pub mod stores;
pub mod stack;
pub mod math;
pub mod conversions;
pub mod comparisons;
pub mod control;
pub mod references;
pub mod extended;
pub mod reserved;

// FIXME: Test code address translation
#[macro_export]
macro_rules! compress_addr {
    ($addr:ident) => {
        fn compressed_range(&mut self, this_pc: usize, translation_map: HashMap<usize, usize>) {
            let goal_addr = this_pc as isize + self.$addr;
            self.$addr = (translation_map.get(&this_pc).unwrap() - translation_map.get(&(goal_addr as usize)).unwrap()) as isize;
        }
    };
}

pub trait Instruction : core::fmt::Debug {
    fn name(&self) -> &'static str;
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, _was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized;
    fn execute(&mut self, _ : &mut JVM) -> Result<(), Error> {
        panic!("TODO execution not implemented for {}", self.name());
    }
    fn compressed_range(&mut self, _this_pc: usize, _translation_map: HashMap<usize, usize>) {}
}

impl std::fmt::Display for dyn Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:#?}")
    }
}