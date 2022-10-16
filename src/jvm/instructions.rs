use super::JVM;

use crate::access_macros;
use crate::class::{Class, self};
use crate::constant_pool::{Entry, ReferenceKind};
use crate::errorcodes::{Error, Opcode};
use crate::reference::{Reference, Monitor, object};
use crate::reference::array::Array;
use crate::reference::object::Object;
use crate::value::{Value, VarValue};

use std::rc::Rc;

pub mod constants;

pub trait Instruction {
    fn name(&self) -> &'static str;
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, _was_wide: bool) -> Result<Self, Error> where Self : Sized;
    fn execute(&mut self, _ : &mut JVM) -> Result<(), Error> {
        panic!("TODO execution not implemented for {}", self.name());
    }
}

impl std::fmt::Display for dyn Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
    }
}

