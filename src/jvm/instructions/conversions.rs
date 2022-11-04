use super::*;

#[derive(Debug)]
pub struct I2L {}
impl Instruction for I2L {
    fn name(&self) -> &'static str {
        "i2l"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(I2L {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::I2L)),
        };
        let result = i64::from(*val.as_int()?);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct I2F {}
impl Instruction for I2F {
    fn name(&self) -> &'static str {
        "i2f"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(I2F {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::I2F)),
        };
        let result = *val.as_int()? as f32;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct I2D {}
impl Instruction for I2D {
    fn name(&self) -> &'static str {
        "i2d"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(I2D {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::I2D)),
        };
        let result = *val.as_int()? as f64;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct L2I {}
impl Instruction for L2I {
    fn name(&self) -> &'static str {
        "l2i"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(L2I {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::L2I)),
        };
        let result = (val.as_long()? & 0xffffffff) as i32;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct L2F {}
impl Instruction for L2F {
    fn name(&self) -> &'static str {
        "l2f"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(L2F {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::L2F)),
        };
        let result = *val.as_long()? as f32;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct L2D {}
impl Instruction for L2D {
    fn name(&self) -> &'static str {
        "l2d"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(L2D {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::L2D)),
        };
        let result = *val.as_long()? as f64;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
}

// The next two instructions, f2i and f2l, use the round-to-zero rule, which is not the IEEE-754 default (as far as I'm aware).
// These functions might not work properly, and so will have to be reworked.
#[derive(Debug)]
pub struct F2I {}
impl Instruction for F2I {
    fn name(&self) -> &'static str {
        "f2i"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(F2I {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::F2I)),
        };
        let result = *val.as_float()? as i32;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct F2L {}
impl Instruction for F2L {
    fn name(&self) -> &'static str {
        "f2l"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(F2L {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::F2L)),
        };
        let result = *val.as_float()? as i64;
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
}

#[derive(Debug)]
pub struct F2D {}
impl Instruction for F2D {
    fn name(&self) -> &'static str {
        "f2d"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(F2D {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::F2D)),
        };
        let result = *val.as_float()? as f64;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
}



