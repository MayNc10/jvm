use super::JVM;

use crate::access_macros;
use crate::class::Class;
use crate::constant_pool::Entry;
use crate::errorcodes::{Error, Opcode};
use crate::reference::{Reference, Monitor};
use crate::value::{Value, VarValue};

use std::rc::Rc;

pub mod constants;
pub mod loads;
pub mod stores;
pub mod stack;
pub mod math;
pub mod conversions;

pub trait Instruction : core::fmt::Debug {
    fn name(&self) -> &'static str;
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, _was_wide: bool) -> Result<Self, Error> where Self : Sized;
    fn execute(&mut self, _ : &mut JVM) -> Result<(), Error> {
        panic!("TODO execution not implemented for {}", self.name());
    }
}

impl std::fmt::Display for dyn Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:#?}")
    }
}


