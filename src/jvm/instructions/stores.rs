use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct IStore {
    idx: usize
}
impl Instruction for IStore {
    fn name(&self) -> &'static str {
        "istore"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
        Ok(IStore { idx })
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISTORE)),
        };
        frame.insert_local(VarValue::Int(*val.as_int()?), self.idx);
        Ok(())
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
pub struct LStore {
    idx: usize
}
impl Instruction for LStore {
    fn name(&self) -> &'static str {
        "lstore"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
        Ok(LStore { idx })
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSTORE)),
        };
        frame.insert_local(VarValue::Long(*val.as_long()?), self.idx);
        Ok(())
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
pub struct FStore {
    idx: usize
}
impl Instruction for FStore {
    fn name(&self) -> &'static str {
        "fstore"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
        Ok(FStore { idx })
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSTORE)),
        };
        frame.insert_local(VarValue::Float(*val.as_float()?), self.idx);
        Ok(())
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
pub struct DStore {
    idx: usize
}
impl Instruction for DStore {
    fn name(&self) -> &'static str {
        "dstore"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
        Ok(DStore { idx })
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSTORE)),
        };
        frame.insert_local(VarValue::Double(*val.as_double()?), self.idx);
        Ok(())
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
pub struct AStore {
    idx: usize
}
impl Instruction for AStore {
    fn name(&self) -> &'static str {
        "astore"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
        Ok(AStore { idx })
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ASTORE)),
        };
        frame.insert_local(VarValue::Reference(val.as_reference()?.clone()), self.idx);
        Ok(())
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
pub struct IStore0 {}
impl Instruction for IStore0 {
    fn name(&self) -> &'static str {
        "istore_0"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IStore0 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISTORE0)),
        };
        frame.insert_local(VarValue::Int(*val.as_int()?), 0);
        Ok(())
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
pub struct IStore1 {}
impl Instruction for IStore1 {
    fn name(&self) -> &'static str {
        "istore_1"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IStore1 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISTORE1)),
        };
        frame.insert_local(VarValue::Int(*val.as_int()?), 1);
        Ok(())
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
    fn can_jit(&self) -> bool { true }
    #[cfg(not(target_family = "wasm"))]
    fn jit(&self, context: &'static Context, module: &Module<'static>, builder: &Builder<'static>, 
                engine: &ExecutionEngine<'static>, name: &String, func: FunctionValue, 
                locals: &Vec<PointerValue>, blocks: &HashMap<usize, BasicBlock>, stack: &PointerValue, top: &PointerValue) 
    {
        builder.build_store(
            locals[1], 
            builder.build_load(unsafe { 
                builder.build_in_bounds_gep(*stack, &[
                    builder.build_load(*top, "top_idx").into_int_value().const_sub(
                        context.i64_type().const_int(1, false)
                    )
                ], "stack_idx")
            }, "top")
        );

        builder.build_store(*top, 
            builder.build_load(*top, "top_idx").into_int_value().const_add(
                context.i64_type().const_int(1, false)
            ), 
        );
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IStore2 {}
impl Instruction for IStore2 {
    fn name(&self) -> &'static str {
        "istore_2"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IStore2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISTORE2)),
        };
        frame.insert_local(VarValue::Int(*val.as_int()?), 2);
        Ok(())
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
pub struct IStore3 {}
impl Instruction for IStore3 {
    fn name(&self) -> &'static str {
        "istore_3"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IStore3 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISTORE3)),
        };
        frame.insert_local(VarValue::Int(*val.as_int()?), 3);
        Ok(())
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
pub struct LStore0 {}
impl Instruction for LStore0 {
    fn name(&self) -> &'static str {
        "lstore_0"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LStore0 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSTORE0)),
        };
        frame.insert_local(VarValue::Long(*val.as_long()?), 0);
        Ok(())
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
pub struct LStore1 {}
impl Instruction for LStore1 {
    fn name(&self) -> &'static str {
        "lstore_1"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LStore1 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSTORE1)),
        };
        frame.insert_local(VarValue::Long(*val.as_long()?), 1);
        Ok(())
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
pub struct LStore2 {}
impl Instruction for LStore2 {
    fn name(&self) -> &'static str {
        "lstore_2"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LStore2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSTORE2)),
        };
        frame.insert_local(VarValue::Long(*val.as_long()?), 2);
        Ok(())
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
pub struct LStore3 {}
impl Instruction for LStore3 {
    fn name(&self) -> &'static str {
        "lstore_3"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LStore3 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSTORE3)),
        };
        frame.insert_local(VarValue::Long(*val.as_long()?), 3);
        Ok(())
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
pub struct FStore0 {}
impl Instruction for FStore0 {
    fn name(&self) -> &'static str {
        "fstore_0"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FStore0 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSTORE0)),
        };
        frame.insert_local(VarValue::Float(*val.as_float()?), 0);
        Ok(())
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
pub struct FStore1 {}
impl Instruction for FStore1 {
    fn name(&self) -> &'static str {
        "fstore_1"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FStore1 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSTORE1)),
        };
        frame.insert_local(VarValue::Float(*val.as_float()?), 1);
        Ok(())
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
pub struct FStore2 {}
impl Instruction for FStore2 {
    fn name(&self) -> &'static str {
        "fstore_2"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FStore2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSTORE2)),
        };
        frame.insert_local(VarValue::Float(*val.as_float()?), 2);
        Ok(())
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
pub struct FStore3 {}
impl Instruction for FStore3 {
    fn name(&self) -> &'static str {
        "fstore_3"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FStore3 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSTORE3)),
        };
        frame.insert_local(VarValue::Float(*val.as_float()?), 3);
        Ok(())
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
pub struct DStore0 {}
impl Instruction for DStore0 {
    fn name(&self) -> &'static str {
        "dstore_0"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DStore0 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSTORE0)),
        };
        frame.insert_local(VarValue::Double(*val.as_double()?), 0);
        Ok(())
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
pub struct DStore1 {}
impl Instruction for DStore1 {
    fn name(&self) -> &'static str {
        "dstore_1"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DStore1 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSTORE1)),
        };
        frame.insert_local(VarValue::Double(*val.as_double()?), 1);
        Ok(())
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
pub struct DStore2 {}
impl Instruction for DStore2 {
    fn name(&self) -> &'static str {
        "dstore_2"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DStore2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSTORE2)),
        };
        frame.insert_local(VarValue::Double(*val.as_double()?), 2);
        Ok(())
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
pub struct DStore3 {}
impl Instruction for DStore3 {
    fn name(&self) -> &'static str {
        "dstore_3"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DStore3 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSTORE3)),
        };
        frame.insert_local(VarValue::Double(*val.as_double()?), 3);
        Ok(())
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
pub struct AStore0 {}
impl Instruction for AStore0 {
    fn name(&self) -> &'static str {
        "astore_0"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(AStore0 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ASTORE0)),
        };
        frame.insert_local(VarValue::Reference(val.as_reference()?.clone()), 0);
        Ok(())
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
pub struct AStore1 {}
impl Instruction for AStore1 {
    fn name(&self) -> &'static str {
        "astore_1"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(AStore1 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ASTORE1)),
        };
        frame.insert_local(VarValue::Reference(val.as_reference()?.clone()), 1);
        Ok(())
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
pub struct AStore2 {}
impl Instruction for AStore2 {
    fn name(&self) -> &'static str {
        "astore_2"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(AStore2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ASTORE2)),
        };
        frame.insert_local(VarValue::Reference(val.as_reference()?.clone()), 2);
        Ok(())
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
pub struct AStore3 {}
impl Instruction for AStore3 {
    fn name(&self) -> &'static str {
        "astore_3"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(AStore3 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ASTORE3)),
        };
        frame.insert_local(VarValue::Reference(val.as_reference()?.clone()), 3);
        Ok(())
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
pub struct IAStore {}
impl Instruction for IAStore {
    fn name(&self) -> &'static str {
        "iastore"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IAStore {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IASTORE)),
        };
        if !val.is_int() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::IASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IASTORE)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let mut array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::IASTORE)),
        };
        if !array.is_iarray() {
            return Err(Error::IncorrectReferenceType(Opcode::IASTORE));
        }
        unsafe {Rc::get_mut_unchecked(&mut array)}.set(*index.as_int()? as usize, val)?;
        Ok(())
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
pub struct LAStore {}
impl Instruction for LAStore {
    fn name(&self) -> &'static str {
        "lastore"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LAStore {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LASTORE)),
        };
        if !val.is_long() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::LASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LASTORE)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let mut array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::LASTORE)),
        };
        if !array.is_larray() {
            return Err(Error::IncorrectReferenceType(Opcode::LASTORE));
        }
        unsafe {Rc::get_mut_unchecked(&mut array)}.set(*index.as_long()? as usize, val)?;
        Ok(())
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
pub struct FAStore {}
impl Instruction for FAStore {
    fn name(&self) -> &'static str {
        "fastore"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FAStore {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FASTORE)),
        };
        if !val.is_float() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::FASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FASTORE)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let mut array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::FASTORE)),
        };
        if !array.is_farray() {
            return Err(Error::IncorrectReferenceType(Opcode::FASTORE));
        }
        unsafe {Rc::get_mut_unchecked(&mut array)}.set(*index.as_int()? as usize, val)?;
        Ok(())
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
pub struct DAStore {}
impl Instruction for DAStore {
    fn name(&self) -> &'static str {
        "dastore"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DAStore {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DASTORE)),
        };
        if !val.is_double() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::DASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DASTORE)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let mut array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::DASTORE)),
        };
        if !array.is_darray() {
            return Err(Error::IncorrectReferenceType(Opcode::DASTORE));
        }
        unsafe {Rc::get_mut_unchecked(&mut array)}.set(*index.as_int()? as usize, val)?;
        Ok(())
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
pub struct AAStore {}
impl Instruction for AAStore {
    fn name(&self) -> &'static str {
        "aastore"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(AAStore {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::AASTORE)),
        };
        if !val.is_reference() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::AASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::AASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::AASTORE)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let mut array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::AASTORE)),
        };
        if !array.is_refarray() {
            return Err(Error::IncorrectReferenceType(Opcode::AASTORE));
        }
        unsafe {Rc::get_mut_unchecked(&mut array)}.set(*index.as_int()? as usize, val)?;
        Ok(())
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
pub struct BAStore {}
impl Instruction for BAStore {
    fn name(&self) -> &'static str {
        "bastore"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(BAStore {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::BASTORE)),
        };
        if !val.is_int() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::BASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::BASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::BASTORE)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let mut array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::BASTORE)),
        };
        if !array.is_barray() | !array.is_boolarray() {
            return Err(Error::IncorrectReferenceType(Opcode::BASTORE));
        }
        unsafe {Rc::get_mut_unchecked(&mut array)}.set(*index.as_int()? as usize, val)?;
        Ok(())
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
pub struct CAStore {}
impl Instruction for CAStore {
    fn name(&self) -> &'static str {
        "castore"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(CAStore {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::CASTORE)),
        };
        if !val.is_int() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::CASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::CASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::CASTORE)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let mut array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::CASTORE)),
        };
        if !array.is_carray() {
            return Err(Error::IncorrectReferenceType(Opcode::CASTORE));
        }
        unsafe {Rc::get_mut_unchecked(&mut array)}.set(*index.as_int()? as usize, val)?;
        Ok(())
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
pub struct SAStore {}
impl Instruction for SAStore {
    fn name(&self) -> &'static str {
        "sastore"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(SAStore {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::SASTORE)),
        };
        if !val.is_int() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::SASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::SASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::SASTORE)),
        };
        let arrayref_rc = arrayref.as_reference_mut()?;
        let mut array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::SASTORE)),
        };
        if !array.is_sarray() {
            return Err(Error::IncorrectReferenceType(Opcode::SASTORE));
        }
        unsafe {Rc::get_mut_unchecked(&mut array)}.set(*index.as_int()? as usize, val)?;
        Ok(())
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