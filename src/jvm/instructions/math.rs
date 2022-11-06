use super::*;

#[derive(Debug)]
pub struct IAdd {}
impl Instruction for IAdd {
    fn name(&self) -> &'static str {
        "iadd"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IAdd {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IADD)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IADD)),
        };
        let (result, _) = val1.as_int()?.overflowing_add(*val2.as_int()?);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LAdd {}
impl Instruction for LAdd {
    fn name(&self) -> &'static str {
        "ladd"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LAdd {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LADD)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LADD)),
        };
        let (result, _) = val1.as_long()?.overflowing_add(*val2.as_long()?);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FAdd {}
impl Instruction for FAdd {
    fn name(&self) -> &'static str {
        "fadd"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FAdd {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FADD)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FADD)),
        };
        let result = val1.as_float()? + val2.as_float()?;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DAdd {}
impl Instruction for DAdd {
    fn name(&self) -> &'static str {
        "dadd"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DAdd {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DADD)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DADD)),
        };
        let result = val1.as_double()? + val2.as_double()?;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct ISub {}
impl Instruction for ISub {
    fn name(&self) -> &'static str {
        "isub"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(ISub {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISUB)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISUB)),
        };
        let (result, _) = val1.as_int()?.overflowing_sub(*val2.as_int()?);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LSub {}
impl Instruction for LSub {
    fn name(&self) -> &'static str {
        "lsub"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LSub {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSUB)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSUB)),
        };
        let (result, _) = val1.as_long()?.overflowing_sub(*val2.as_long()?);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FSub {}
impl Instruction for FSub {
    fn name(&self) -> &'static str {
        "fsub"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FSub {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSUB)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSUB)),
        };
        let result = val1.as_float()? - val2.as_float()?;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DSub {}
impl Instruction for DSub {
    fn name(&self) -> &'static str {
        "dsub"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DSub {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSUB)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSUB)),
        };
        let result = val1.as_double()? - val2.as_double()?;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct IMul {}
impl Instruction for IMul {
    fn name(&self) -> &'static str {
        "imul"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IMul {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IMUL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IMUL)),
        };
        let (result, _) = val1.as_int()?.overflowing_mul(*val2.as_int()?);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LMul {}
impl Instruction for LMul {
    fn name(&self) -> &'static str {
        "lmul"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LMul {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LMUL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LMUL)),
        };
        let (result, _) = val1.as_long()?.overflowing_mul(*val2.as_long()?);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FMul {}
impl Instruction for FMul {
    fn name(&self) -> &'static str {
        "fmul"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FMul {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FMUL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FMUL)),
        };
        let result = val1.as_float()? * val2.as_float()?;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DMul {}
impl Instruction for DMul {
    fn name(&self) -> &'static str {
        "dmul"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DMul {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DMUL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DMUL)),
        };
        let result = val1.as_double()? * val2.as_double()?;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct IDiv {}
impl Instruction for IDiv {
    fn name(&self) -> &'static str {
        "idiv"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IDiv {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IDIV)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IDIV)),
        };
        if *val2.as_int()? == 0 {
            return Err(Error::ArithmeticException(Opcode::IDIV));
        }
        let (result, _) = val1.as_int()?.overflowing_div(*val2.as_int()?);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LDiv {}
impl Instruction for LDiv {
    fn name(&self) -> &'static str {
        "ldiv"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LDiv {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LDIV)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LDIV)),
        };
        if *val2.as_long()? == 0 {
            return Err(Error::ArithmeticException(Opcode::IDIV));
        }
        let (result, _) = val1.as_long()?.overflowing_div(*val2.as_long()?);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FDiv {}
impl Instruction for FDiv {
    fn name(&self) -> &'static str {
        "fdiv"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FDiv {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FDIV)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FDIV)),
        };
        let result = val1.as_float()? / val2.as_float()?;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DDiv {}
impl Instruction for DDiv {
    fn name(&self) -> &'static str {
        "ddiv"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DDiv {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DDIV)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DDIV)),
        };
        let result = val1.as_double()? / val2.as_double()?;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct IRem {}
impl Instruction for IRem {
    fn name(&self) -> &'static str {
        "irem"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IRem {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IREM)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IREM)),
        };
        if *val2.as_int()? == 0 {
            return Err(Error::ArithmeticException(Opcode::IREM));
        }
        let (result, _) = val1.as_int()?.overflowing_rem(*val2.as_int()?);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LRem {}
impl Instruction for LRem {
    fn name(&self) -> &'static str {
        "lrem"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LRem {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LREM)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LREM)),
        };
        if *val2.as_long()? == 0 {
            return Err(Error::ArithmeticException(Opcode::IREM));
        }
        let (result, _) = val1.as_long()?.overflowing_rem(*val2.as_long()?);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FRem {}
impl Instruction for FRem {
    fn name(&self) -> &'static str {
        "frem"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FRem {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FREM)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FREM)),
        };
        let result = val1.as_float()? % val2.as_float()?;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DRem {}
impl Instruction for DRem {
    fn name(&self) -> &'static str {
        "drem"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DRem {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DREM)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DREM)),
        };
        let result = val1.as_double()? % val2.as_double()?;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct INeg {}
impl Instruction for INeg {
    fn name(&self) -> &'static str {
        "ineg"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(INeg {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::INEG)),
        };
        let (result, _) = val.as_int()?.overflowing_neg();
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LNeg {}
impl Instruction for LNeg {
    fn name(&self) -> &'static str {
        "lneg"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LNeg {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LNEG)),
        };
        let (result, _) = val.as_long()?.overflowing_neg();
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FNeg {}
impl Instruction for FNeg {
    fn name(&self) -> &'static str {
        "fneg"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FNeg {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FNEG)),
        };
        let result = val.as_float()? * -1.0;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DNeg {}
impl Instruction for DNeg {
    fn name(&self) -> &'static str {
        "dneg"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DNeg {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DNEG)),
        };
        let result = val.as_double()? * -1.0;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct IShl {}
impl Instruction for IShl {
    fn name(&self) -> &'static str {
        "ishl"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IShl {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISHL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISHL)),
        };
        let result = val1.as_int()? << (val2.as_int()? & 0x1f);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LShl {}
impl Instruction for LShl {
    fn name(&self) -> &'static str {
        "lshl"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LShl {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSHL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSHL)),
        };
        let result = val1.as_long()? << (val2.as_long()? & 0x3f);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct IShr {}
impl Instruction for IShr {
    fn name(&self) -> &'static str {
        "ishr"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IShr {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISHR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISHR)),
        };
        let result = val1.as_int()? >> (val2.as_int()? & 0x1f);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LShr {}
impl Instruction for LShr {
    fn name(&self) -> &'static str {
        "lshr"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LShr {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSHR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSHR)),
        };
        let result = val1.as_long()? >> (val2.as_long()? & 0x3f);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct IUshr {}
impl Instruction for IUshr {
    fn name(&self) -> &'static str {
        "iushr"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IUshr {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IUSHR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IUSHR)),
        };
        let mut result = val1.as_int()? >> (val2.as_int()? & 0x1f);
        if *val1.as_int()? < 0 {
            // Account for propagated sign bit.
            // See: https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-6.html#jvms-6.5.fadd
            result += 2 << !(val2.as_int()? & 0x1f);
        }
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LUshr {}
impl Instruction for LUshr {
    fn name(&self) -> &'static str {
        "lushr"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LUshr {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LUSHR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LUSHR)),
        };
        let mut result = val1.as_long()? >> (val2.as_long()? & 0x3f);
        if *val1.as_long()? < 0 {
            // Account for propagated sign bit.
            // See: https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-6.html#jvms-6.5.fadd
            result += 2 << !(val2.as_long()? & 0x3f);
        }
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct IAnd {}
impl Instruction for IAnd {
    fn name(&self) -> &'static str {
        "iand"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IAnd {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IAND)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IAND)),
        };
        let result = val1.as_int()? & (val2.as_int()? & 0x1f);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LAnd {}
impl Instruction for LAnd {
    fn name(&self) -> &'static str {
        "land"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LAnd {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LAND)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LAND)),
        };
        let result = val1.as_long()? & (val2.as_long()? & 0x3f);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct IOr {}
impl Instruction for IOr {
    fn name(&self) -> &'static str {
        "ior"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IOr {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IOR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IOR)),
        };
        let result = val1.as_int()? | (val2.as_int()? & 0x1f);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LOr {}
impl Instruction for LOr {
    fn name(&self) -> &'static str {
        "lor"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LOr {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LOR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LOR)),
        };
        let result = val1.as_long()? | (val2.as_long()? & 0x3f);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct IXor {}
impl Instruction for IXor {
    fn name(&self) -> &'static str {
        "ixor"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IXor {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IXOR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IXOR)),
        };
        let result = val1.as_int()? ^ (val2.as_int()? & 0x1f);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LXor {}
impl Instruction for LXor {
    fn name(&self) -> &'static str {
        "lxor"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LXor {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LXOR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LXOR)),
        };
        let result = val1.as_long()? ^ (val2.as_long()? & 0x3f);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct IInc {
    index: usize,
    const_incr: i32,
}
impl Instruction for IInc {
    fn name(&self) -> &'static str {
        "iinc"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = v[0] as usize;
            let const_incr = v[1] as i32;
            v.remove(0);
            v.remove(0);
            Ok(IInc {index, const_incr})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &mut frame.local_variables[self.index];
        // Confirm that this updates the value.
        *var.as_int_mut()? += self.const_incr;
        Ok(())
    }
}