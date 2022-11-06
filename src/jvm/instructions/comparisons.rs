use super::*;
use crate::compress_addr;

#[derive(Debug)]
pub struct LCmp {}
impl Instruction for LCmp {
    fn name(&self) -> &'static str {
        "lcmp"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LCmp {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LCMP)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LCMP)),
        };
        let result = match val1.as_long()? > val2.as_long()? {
            true => 1_i32,
            false => match val1.as_long()? < val2.as_long()? {
                true => -1_i32,
                false => 0_i32,
            }
        };
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FCmpL {}
impl Instruction for FCmpL {
    fn name(&self) -> &'static str {
        "fcmpl"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FCmpL {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FCMPL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FCMPL)),
        };
        // Because this is a float comparison, we have to deal with NaNs.
        // in this function, we group them together with the < case and return -1.
        let result = match val1.as_float()? > val2.as_float()? {
            true => 1_i32,
            false => match val1.as_float()? == val2.as_float()? {
                true => 0_i32,
                false => -1_i32,
            }
        };
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FCmpG {}
impl Instruction for FCmpG {
    fn name(&self) -> &'static str {
        "fcmpg"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FCmpG {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FCMPG)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FCMPG)),
        };
        // Because this is a float comparison, we have to deal with NaNs.
        // in this function, we group them together with the > case and return 1.
        let result = match val1.as_float()? < val2.as_float()? {
            true => -1_i32,
            false => match val1.as_float()? == val2.as_float()? {
                true => 0_i32,
                false => 1_i32,
            }
        };
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DCmpL {}
impl Instruction for DCmpL {
    fn name(&self) -> &'static str {
        "dcmpl"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DCmpL {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DCMPL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DCMPL)),
        };
        // Because this is a double comparison, we have to deal with NaNs.
        // in this function, we group them together with the < case and return -1.
        let result = match val1.as_double()? > val2.as_double()? {
            true => 1_i32,
            false => match val1.as_double()? == val2.as_double()? {
                true => 0_i32,
                false => -1_i32,
            }
        };
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DcmpG {}
impl Instruction for DcmpG {
    fn name(&self) -> &'static str {
        "dcmpg"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DcmpG {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DCMPG)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DCMPG)),
        };
        // Because this is a double comparison, we have to deal with NaNs.
        // in this function, we group them together with the > case and return 1.
        let result = match val1.as_double()? < val2.as_double()? {
            true => -1_i32,
            false => match val1.as_double()? == val2.as_double()? {
                true => 0_i32,
                false => 1_i32,
            }
        };
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct IfEq {
    offset: isize,
}
impl Instruction for IfEq {
    fn name(&self) -> &'static str {
        "ifeq"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfEq {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFEQ)),
            };
            match *val.as_int()? == 0 {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct IfNe {
    offset: isize,
}
impl Instruction for IfNe {
    fn name(&self) -> &'static str {
        "ifne"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfNe {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFNE)),
            };
            match *val.as_int()? != 0 {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct IfLt {
    offset: isize,
}
impl Instruction for IfLt {
    fn name(&self) -> &'static str {
        "iflt"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfLt {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFLT)),
            };
            match *val.as_int()? < 0 {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct IfGe {
    offset: isize,
}
impl Instruction for IfGe {
    fn name(&self) -> &'static str {
        "ifge"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfGe {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFGE)),
            };
            match *val.as_int()? >= 0 {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct IfGt {
    offset: isize,
}
impl Instruction for IfGt {
    fn name(&self) -> &'static str {
        "ifgt"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfGt {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFGT)),
            };
            match *val.as_int()? > 0 {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct IfLe {
    offset: isize,
}
impl Instruction for IfLe {
    fn name(&self) -> &'static str {
        "ifle"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfLe {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFLE)),
            };
            match *val.as_int()? <= 0 {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct IfICmpEq {
    offset: isize,
}
impl Instruction for IfICmpEq {
    fn name(&self) -> &'static str {
        "if_icmpeq"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfICmpEq {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPEQ)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPEQ)),
            };
            match val1.as_int()? == val2.as_int()? {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct IfICmpNe {
    offset: isize,
}
impl Instruction for IfICmpNe {
    fn name(&self) -> &'static str {
        "if_icmpne"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfICmpNe {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPNE)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPNE)),
            };
            match val1.as_int()? != val2.as_int()? {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct IfICmpLt {
    offset: isize,
}
impl Instruction for IfICmpLt {
    fn name(&self) -> &'static str {
        "if_icmplt"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfICmpLt {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPLT)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPLT)),
            };
            match val1.as_int()? < val2.as_int()? {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct IfICmpGe {
    offset: isize,
}
impl Instruction for IfICmpGe {
    fn name(&self) -> &'static str {
        "if_icmpge"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfICmpGe {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPGE)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPGE)),
            };
            match val1.as_int()? >= val2.as_int()? {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct IfICmpGt {
    offset: isize,
}
impl Instruction for IfICmpGt {
    fn name(&self) -> &'static str {
        "if_icmpgt"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfICmpGt {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPGT)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPGT)),
            };
            match val1.as_int()? >= val2.as_int()? {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct IfICmpLe {
    offset: isize,
}
impl Instruction for IfICmpLe {
    fn name(&self) -> &'static str {
        "if_icmple"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfICmpLe {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPLE)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPLE)),
            };
            match val1.as_int()? <= val2.as_int()? {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct IfACmpEq {
    offset: isize,
}
impl Instruction for IfACmpEq {
    fn name(&self) -> &'static str {
        "if_acmpeq"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfACmpEq {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPEQ)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPEQ)),
            };
            match Reference::ptr_eq(&val1.as_reference()?, &val2.as_reference()?) {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}

#[derive(Debug)]
pub struct IfACmpNe {
    offset: isize,
}
impl Instruction for IfACmpNe {
    fn name(&self) -> &'static str {
        "if_acmpne"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            };
            Ok(IfACmpNe {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPNE)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPNE)),
            };
            !match Reference::ptr_eq(&val1.as_reference()?, &val2.as_reference()?) {
                true => self.offset,
                false => 1,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    compress_addr!{offset}
}
