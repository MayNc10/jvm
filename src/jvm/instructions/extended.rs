use crate::reference::array::Array;

use super::*;
use crate::compress_addr;

#[derive(Debug)]
pub struct Wide {}
impl Instruction for Wide {
    fn name(&self) -> &'static str {
        "wide"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Err(Error::Wide)
        }
    }
    fn execute(&mut self, _jvm : &mut JVM) -> Result<(), Error> {
        Err(Error::Wide)
    }
}

#[derive(Debug)]
pub struct MultiANewArray {
    index: u16,
    dimensions: usize,
}
impl Instruction for MultiANewArray {
    fn name(&self) -> &'static str {
        "multianewarray"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            let dimensions = v[2] as usize;
            v.remove(0); v.remove(0); v.remove(0);
            Ok(MultiANewArray { index, dimensions })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        if self.dimensions == 0 {
            return Err(Error::IllegalDimensionCount);
        }
        let len = frame.op_stack.len();
        if len >= self.dimensions {
            return Err(Error::NotEnoughDimensionValues);
        }
        let counts = &frame.op_stack[(len - self.dimensions)..0];
        let array_class_desc_index = *frame.rt_const_pool.get_class_file().cp_entry(self.index)?.as_class()?;
        let array_class_desc = frame.rt_const_pool.get_class_file().cp_entry(array_class_desc_index)?.as_utf8()?.clone();
        let array = Array::new_multi(self.dimensions as u8, counts, array_class_desc)?;
        let array_ref = Reference::Array(Rc::new(array), Rc::new(Monitor::new()));
        let array_ref_val = Value::Reference(array_ref);
        frame.op_stack.push(array_ref_val);
        Ok(())
    }
}

#[derive(Debug)]
pub struct IfNull {
    offset: isize,
}
impl Instruction for IfNull {
    fn name(&self) -> &'static str {
        "ifnull"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(IfNull {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let value = match frame.op_stack.pop() {
            Some(v) => v.as_reference()?,
            None => return Err(Error::StackUnderflow(Opcode::IFNONNULL)),
        };
        if let Reference::Null = value {
            thread.inc_pc(self.offset)?;
        }
        Ok(())
    }
    compress_addr!(offset);
}

#[derive(Debug)]
pub struct IfNonNull {
    offset: isize,
}
impl Instruction for IfNonNull {
    fn name(&self) -> &'static str {
        "ifnonnull"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(IfNonNull {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let value = match frame.op_stack.pop() {
            Some(v) => v.as_reference()?,
            None => return Err(Error::StackUnderflow(Opcode::IFNONNULL)),
        };
        if !matches!(value, Reference::Null) {
            thread.inc_pc(self.offset)?;
        }
        Ok(())
    }
    compress_addr!(offset);
}

#[derive(Debug)]
pub struct GotoW {
    offset: isize,
}
impl Instruction for GotoW {
    fn name(&self) -> &'static str {
        "goto_w"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 4).try_into().unwrap())
            };
            v.remove(0); v.remove(0); v.remove(0); v.remove(0);
            Ok(GotoW {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        thread.inc_pc(self.offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct JsrW {
    offset: isize,
}
impl Instruction for JsrW {
    fn name(&self) -> &'static str {
        "jsr_w"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 4).try_into().unwrap())
            };
            v.remove(0); v.remove(0); v.remove(0); v.remove(0);
            Ok(JsrW {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let pc = thread.pc() as u16;
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::ReturnAddress(pc));
        thread.inc_pc(self.offset)?;
        Ok(())
    }
    compress_addr!{offset}
}