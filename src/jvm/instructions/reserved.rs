use super::*;

#[derive(Debug)]
pub struct Breakpoint {}
impl Instruction for Breakpoint {
    fn name(&self) -> &'static str {
        "breakpoint"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Err(Error::Breakpoint)
        }
    }
    fn execute(&mut self, _jvm : &mut JVM) -> Result<(), Error> {
        Err(Error::Breakpoint)
    }
}

#[derive(Debug)]
pub struct ImpDep1 {}
impl Instruction for ImpDep1 {
    fn name(&self) -> &'static str {
        "impdep1"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Err(Error::ImpDep1)
        }
    }
    fn execute(&mut self, _jvm : &mut JVM) -> Result<(), Error> {
        Err(Error::ImpDep1)
    }
}

#[derive(Debug)]
pub struct ImpDep2 {}
impl Instruction for ImpDep2 {
    fn name(&self) -> &'static str {
        "impdep2"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Err(Error::ImpDep2)
        }
    }
    fn execute(&mut self, _jvm : &mut JVM) -> Result<(), Error> {
        Err(Error::ImpDep2)
    }
}