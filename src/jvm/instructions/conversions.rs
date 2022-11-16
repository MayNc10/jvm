use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct I2L {}
impl Instruction for I2L {
    fn name(&self) -> &'static str {
        "i2l"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<I2L>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct I2F {}
impl Instruction for I2F {
    fn name(&self) -> &'static str {
        "i2f"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<I2F>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct I2D {}
impl Instruction for I2D {
    fn name(&self) -> &'static str {
        "i2d"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<I2D>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct L2I {}
impl Instruction for L2I {
    fn name(&self) -> &'static str {
        "l2i"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<L2I>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct L2F {}
impl Instruction for L2F {
    fn name(&self) -> &'static str {
        "l2f"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<L2F>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct L2D {}
impl Instruction for L2D {
    fn name(&self) -> &'static str {
        "l2d"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<L2D>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

// The next two instructions, f2i and f2l, use the round-to-zero rule, which is not the IEEE-754 default (as far as I'm aware).
// These functions might not work properly, and so will have to be reworked.
#[derive(Debug, PartialEq, Clone)]
pub struct F2I {}
impl Instruction for F2I {
    fn name(&self) -> &'static str {
        "f2i"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<F2I>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct F2L {}
impl Instruction for F2L {
    fn name(&self) -> &'static str {
        "f2l"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<F2L>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct F2D {}
impl Instruction for F2D {
    fn name(&self) -> &'static str {
        "f2d"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<F2D>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct D2I {}
impl Instruction for D2I {
    fn name(&self) -> &'static str {
        "d2i"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(D2I {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::D2I)),
        };
        let result = *val.as_double()? as i32;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<D2I>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct D2L {}
impl Instruction for D2L {
    fn name(&self) -> &'static str {
        "d2l"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(D2L {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::D2L)),
        };
        let result = *val.as_double()? as i64;
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<D2L>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct D2F {}
impl Instruction for D2F {
    fn name(&self) -> &'static str {
        "d2f"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(D2F {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::D2F)),
        };
        let result = *val.as_double()? as f32;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<D2F>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct I2B {}
impl Instruction for I2B {
    fn name(&self) -> &'static str {
        "i2b"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(I2B {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::I2B)),
        };
        let result = ((val.as_int()? & 0xff) as i8) as i32;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<I2B>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct I2C {}
impl Instruction for I2C {
    fn name(&self) -> &'static str {
        "i2c"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(I2C {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::I2C)),
        };
        let result = ((val.as_int()? & 0xffff) as u16) as i32;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<I2C>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct I2S {}
impl Instruction for I2S {
    fn name(&self) -> &'static str {
        "i2s"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(I2S {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::I2S)),
        };
        let result = ((val.as_int()? & 0xffff) as i16) as i32;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<I2S>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
