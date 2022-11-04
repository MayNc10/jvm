use super::*;

#[derive(Debug)]
pub struct ILoad {
    idx: usize
}
impl Instruction for ILoad {
    fn name(&self) -> &'static str {
        "iload"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        let idx = if was_wide {
            unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) as usize
            }
        } else {
            v[0] as usize
        };
        v.remove(0);
        if was_wide {
            v.remove(0);
        }
        Ok(ILoad { idx })
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[self.idx];
        frame.op_stack.push(Value::Int(*var.as_int()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LLoad {
    idx: usize
}
impl Instruction for LLoad {
    fn name(&self) -> &'static str {
        "lload"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        let idx = if was_wide {
            unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) as usize
            }
        } else {
            v[0] as usize
        };
        v.remove(0);
        if was_wide {
            v.remove(0);
        }
        Ok(LLoad { idx })
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[self.idx];
        frame.op_stack.push(Value::Long(*var.as_long()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FLoad {
    idx: usize
}
impl Instruction for FLoad {
    fn name(&self) -> &'static str {
        "fload"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        let idx = if was_wide {
            unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) as usize
            }
        } else {
            v[0] as usize
        };
        v.remove(0);
        if was_wide {
            v.remove(0);
        }
        Ok(FLoad { idx })
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[self.idx];
        frame.op_stack.push(Value::Float(*var.as_float()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DLoad {
    idx: usize
}
impl Instruction for DLoad {
    fn name(&self) -> &'static str {
        "dload"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        let idx = if was_wide {
            unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) as usize
            }
        } else {
            v[0] as usize
        };
        v.remove(0);
        if was_wide {
            v.remove(0);
        }
        Ok(DLoad { idx })
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[self.idx];
        frame.op_stack.push(Value::Long(*var.as_long()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct ALoad {
    idx: usize
}
impl Instruction for ALoad {
    fn name(&self) -> &'static str {
        "aload"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        let idx = if was_wide {
            unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) as usize
            }
        } else {
            v[0] as usize
        };
        v.remove(0);
        if was_wide {
            v.remove(0);
        }
        Ok(ALoad { idx })
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[self.idx];
        frame.op_stack.push(Value::Reference(var.as_reference()?.clone()));
        Ok(())
    }
}

#[derive(Debug)]
pub struct ILoad0 {}
impl Instruction for ILoad0 {
    fn name(&self) -> &'static str {
        "iload_0"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(ILoad0 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[0];
        frame.op_stack.push(Value::Int(*var.as_int()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct ILoad1 {}
impl Instruction for ILoad1 {
    fn name(&self) -> &'static str {
        "iload_1"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(ILoad1 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[1];
        frame.op_stack.push(Value::Int(*var.as_int()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct ILoad2 {}
impl Instruction for ILoad2 {
    fn name(&self) -> &'static str {
        "iload_2"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(ILoad2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[2];
        frame.op_stack.push(Value::Int(*var.as_int()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct ILoad3 {}
impl Instruction for ILoad3 {
    fn name(&self) -> &'static str {
        "iload_3"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(ILoad3 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[3];
        frame.op_stack.push(Value::Int(*var.as_int()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LLoad0 {}
impl Instruction for LLoad0 {
    fn name(&self) -> &'static str {
        "lload_0"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LLoad0 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[0];
        frame.op_stack.push(Value::Long(*var.as_long()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LLoad1 {}
impl Instruction for LLoad1 {
    fn name(&self) -> &'static str {
        "lload_1"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LLoad1 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[1];
        frame.op_stack.push(Value::Long(*var.as_long()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LLoad2 {}
impl Instruction for LLoad2 {
    fn name(&self) -> &'static str {
        "lload_2"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LLoad2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[2];
        frame.op_stack.push(Value::Long(*var.as_long()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LLoad3 {}
impl Instruction for LLoad3 {
    fn name(&self) -> &'static str {
        "lload_3"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LLoad3 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[3];
        frame.op_stack.push(Value::Long(*var.as_long()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FLoad0 {}
impl Instruction for FLoad0 {
    fn name(&self) -> &'static str {
        "fload_0"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FLoad0 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[0];
        frame.op_stack.push(Value::Float(*var.as_float()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FLoad1 {}
impl Instruction for FLoad1 {
    fn name(&self) -> &'static str {
        "fload_1"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FLoad1 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[1];
        frame.op_stack.push(Value::Float(*var.as_float()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FLoad2 {}
impl Instruction for FLoad2 {
    fn name(&self) -> &'static str {
        "fload_2"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FLoad2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[2];
        frame.op_stack.push(Value::Float(*var.as_float()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FLoad3 {}
impl Instruction for FLoad3 {
    fn name(&self) -> &'static str {
        "fload_3"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FLoad3 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[3];
        frame.op_stack.push(Value::Float(*var.as_float()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DLoad0 {}
impl Instruction for DLoad0 {
    fn name(&self) -> &'static str {
        "dload_0"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DLoad0 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[0];
        frame.op_stack.push(Value::Double(*var.as_double()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DLoad1 {}
impl Instruction for DLoad1 {
    fn name(&self) -> &'static str {
        "dload_1"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DLoad1 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[1];
        frame.op_stack.push(Value::Double(*var.as_double()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DLoad2 {}
impl Instruction for DLoad2 {
    fn name(&self) -> &'static str {
        "dload_2"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DLoad2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[2];
        frame.op_stack.push(Value::Double(*var.as_double()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DLoad3 {}
impl Instruction for DLoad3 {
    fn name(&self) -> &'static str {
        "dload_3"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DLoad3 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[3];
        frame.op_stack.push(Value::Double(*var.as_double()?));
        Ok(())
    }
}

#[derive(Debug)]
pub struct ALoad0 {}
impl Instruction for ALoad0 {
    fn name(&self) -> &'static str {
        "aload_0"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(ALoad0 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[0];
        frame.op_stack.push(Value::Reference(var.as_reference()?.clone()));
        Ok(())
    }
}

#[derive(Debug)]
pub struct ALoad1 {}
impl Instruction for ALoad1 {
    fn name(&self) -> &'static str {
        "aload_1"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(ALoad1 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[1];
        frame.op_stack.push(Value::Reference(var.as_reference()?.clone()));
        Ok(())
    }
}

#[derive(Debug)]
pub struct ALoad2 {}
impl Instruction for ALoad2 {
    fn name(&self) -> &'static str {
        "aload_2"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(ALoad2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[2];
        frame.op_stack.push(Value::Reference(var.as_reference()?.clone()));
        Ok(())
    }
}

#[derive(Debug)]
pub struct ALoad3 {}
impl Instruction for ALoad3 {
    fn name(&self) -> &'static str {
        "aload_3"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(ALoad3 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[3];
        frame.op_stack.push(Value::Reference(var.as_reference()?.clone()));
        Ok(())
    }
}

#[derive(Debug)]
pub struct IALoad {}
impl Instruction for IALoad {
    fn name(&self) -> &'static str {
        "iaload"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IALoad {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::IALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::IALOAD)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::IALOAD)),
        };
        if !arr.is_iarray() {
            return Err(Error::IncorrectReferenceType(Opcode::IALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LALoad {}
impl Instruction for LALoad {
    fn name(&self) -> &'static str {
        "laload"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LALoad {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::LALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::LALOAD)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::LALOAD)),
        };
        if !arr.is_larray() {
            return Err(Error::IncorrectReferenceType(Opcode::LALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
}

#[derive(Debug)]
pub struct FALoad {}
impl Instruction for FALoad {
    fn name(&self) -> &'static str {
        "faload"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FALoad {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::FALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::FALOAD)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::FALOAD)),
        };
        if !arr.is_farray() {
            return Err(Error::IncorrectReferenceType(Opcode::FALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DALoad {}
impl Instruction for DALoad {
    fn name(&self) -> &'static str {
        "daload"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DALoad {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::DALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::DALOAD)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::DALOAD)),
        };
        if !arr.is_darray() {
            return Err(Error::IncorrectReferenceType(Opcode::DALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
}

#[derive(Debug)]
pub struct AALoad {}
impl Instruction for AALoad {
    fn name(&self) -> &'static str {
        "aaload"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(AALoad {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::AALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::AALOAD)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::AALOAD)),
        };
        if !arr.is_refarray() {
            return Err(Error::IncorrectReferenceType(Opcode::AALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
}

#[derive(Debug)]
pub struct BALoad {}
impl Instruction for BALoad {
    fn name(&self) -> &'static str {
        "baload"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(BALoad {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::BALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::BALOAD)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::BALOAD)),
        };
        if !arr.is_barray() | !arr.is_boolarray() {
            return Err(Error::IncorrectReferenceType(Opcode::BALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
}

#[derive(Debug)]
pub struct CALoad {}
impl Instruction for CALoad {
    fn name(&self) -> &'static str {
        "caload"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(CALoad {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::CALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::CALOAD)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::CALOAD)),
        };
        if !arr.is_carray() {
            return Err(Error::IncorrectReferenceType(Opcode::CALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
}

#[derive(Debug)]
pub struct SALoad {}
impl Instruction for SALoad {
    fn name(&self) -> &'static str {
        "saload"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(SALoad {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::SALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::SALOAD)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::SALOAD)),
        };
        if !arr.is_sarray() {
            return Err(Error::IncorrectReferenceType(Opcode::SALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
}