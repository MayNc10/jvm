use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Breakpoint {}
impl Instruction for Breakpoint {
    fn name(&self) -> &'static str {
        "breakpoint"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Err(Error::Breakpoint)
        }
    }
    fn execute(&mut self, _jvm : &mut JVM) -> Result<(), Error> {
        Err(Error::Breakpoint)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImpDep1 {}
impl Instruction for ImpDep1 {
    fn name(&self) -> &'static str {
        "impdep1"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Err(Error::ImpDep1)
        }
    }
    fn execute(&mut self, _jvm : &mut JVM) -> Result<(), Error> {
        Err(Error::ImpDep1)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImpDep2 {}
impl Instruction for ImpDep2 {
    fn name(&self) -> &'static str {
        "impdep2"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Err(Error::ImpDep2)
        }
    }
    fn execute(&mut self, _jvm : &mut JVM) -> Result<(), Error> {
        Err(Error::ImpDep2)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}