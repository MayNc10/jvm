use super::*;
use crate::compress_addr;

#[derive(Debug, PartialEq, Clone)]
pub struct Goto {
    offset: isize,
}
impl Instruction for Goto {
    fn name(&self) -> &'static str {
        "goto"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                i16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) as isize
            };
            v.remove(0); v.remove(0);
            Ok(Goto {offset})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        thread.inc_pc(self.offset)?;
        Ok(())
    }
    compress_addr!{offset}
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Goto>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Jsr {
    offset: isize,
}
impl Instruction for Jsr {
    fn name(&self) -> &'static str {
        "jsr"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let offset = unsafe {
                i16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) as isize
            };
            v.remove(0); v.remove(0);
            Ok(Jsr {offset})
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Jsr>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ret {
    index: usize,
}
impl Instruction for Ret {
    fn name(&self) -> &'static str {
        "ret"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = v[0] as usize;
            v.remove(0); v.remove(0);
            Ok(Ret {index})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let addr = {
            let frame = access_macros::current_frame_mut!(thread);
            *frame.local_variables[self.index].as_retaddr()?
        };
        thread.set_pc(addr as usize)?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Ret>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableSwitch {
    default: isize,
    low: isize,
    high: isize,
    j_offsets: Box<[isize]>,
}
impl Instruction for TableSwitch {
    fn name(&self) -> &'static str {
        "tableswitch"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, mut true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            while true_pc % 4 != 0 {
                v.remove(0);
                true_pc += 1;
            }
            let default = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 4).try_into().unwrap())
            };
            let low = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 4).try_into().unwrap())
            };
            let high = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 4).try_into().unwrap())
            };
            v.remove(0); v.remove(0); v.remove(0); v.remove(0); 
            v.remove(0); v.remove(0); v.remove(0); v.remove(0);
            v.remove(0); v.remove(0); v.remove(0); v.remove(0);
            let mut j_offsets = Vec::with_capacity((high - low + 1) as usize / 4).into_boxed_slice();
            for idx in 0..(high - low + 1) as usize {
                j_offsets[idx] = unsafe {
                    isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 4).try_into().unwrap())
                };
                v.remove(0); v.remove(0); v.remove(0); v.remove(0); 
            }
            Ok(TableSwitch { default, low, high, j_offsets })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let index = match frame.op_stack.pop() {
                Some(v) => *v.as_int()?,
                None => return Err(Error::StackUnderflow(Opcode::TABLESWITCH)),
            } as isize;
            match (index >= self.low) && (index <= self.high) {
                true => self.j_offsets[(index - self.low) as usize],
                false => self.default,
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<TableSwitch>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LookupSwitch {
    default: isize,
    pairs: Box<[(i32, isize)]>,
}
impl Instruction for LookupSwitch {
    fn name(&self) -> &'static str {
        "lookupswitch"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, mut true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            while true_pc % 4 != 0 {
                v.remove(0);
                true_pc += 1;
            }
            let default = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 4).try_into().unwrap())
            };
            let npairs = unsafe {
                isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 4).try_into().unwrap())
            };           
            v.remove(0); v.remove(0); v.remove(0); v.remove(0); 
            let mut pairs = Vec::with_capacity(npairs as usize).into_boxed_slice();
            for idx in 0..npairs as usize {
                pairs[idx] = unsafe {
                    (i32::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 4).try_into().unwrap()),
                     isize::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 4).try_into().unwrap()))
                };
                v.remove(0); v.remove(0); v.remove(0); v.remove(0); 
            }
            Ok(LookupSwitch { default, pairs})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let offset = {
            // Maybe cache offsets?
            let frame = access_macros::current_frame_mut!(thread);
            let key = match frame.op_stack.pop() {
                Some(v) => *v.as_int()?,
                None => return Err(Error::StackUnderflow(Opcode::TABLESWITCH)),
            };
            match self.pairs.binary_search_by_key(&key, |&(m, _)| m) {
                Ok(idx) => self.pairs[idx].1,
                Err(_) => self.default 
            }
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<LookupSwitch>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IReturn {}
impl Instruction for IReturn {
    fn name(&self) -> &'static str {
        "ireturn"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(IReturn {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let return_value = {
            let frame = access_macros::current_frame_mut!(thread);
            if !frame.current_method.returns_int(&frame.rt_const_pool.get_class_file())? {
                return Err(Error::IncompatibleReturnType(Opcode::IRETURN));
            }
            let ret_type = frame.current_method.return_char(&frame.rt_const_pool.get_class_file())?;
            let ret_val = match frame.op_stack.pop() {
                Some(v) => *v.as_int()?,
                None => return Err(Error::StackUnderflow(Opcode::IRETURN)),
            };
            match ret_type {
                'B' => Value::Byte((ret_val as i8) as i32),
                'C' => Value::Char((ret_val as u16) as i32),
                'I' => Value::Int(ret_val),
                'S' => Value::Short((ret_val as i16) as i32),
                'Z' => Value::Byte((ret_val == 1) as i32),
                _ => unreachable!(),
            }
        };
        match thread.m_stack.pop() {
            Some(_) => (),
            None => return Err(Error::FrameStackUnderflow(Opcode::IRETURN)),
        }
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(return_value);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<IReturn>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LReturn {}
impl Instruction for LReturn {
    fn name(&self) -> &'static str {
        "lreturn"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(LReturn {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let return_value = {
            let frame = access_macros::current_frame_mut!(thread);
            if !frame.current_method.returns_long(&frame.rt_const_pool.get_class_file())? {
                return Err(Error::IncompatibleReturnType(Opcode::LRETURN));
            }
            match frame.op_stack.pop() {
                Some(v) => Value::Long(*v.as_long()?),
                None => return Err(Error::StackUnderflow(Opcode::LRETURN)),
            }          
        };
        match thread.m_stack.pop() {
            Some(_) => (),
            None => return Err(Error::FrameStackUnderflow(Opcode::LRETURN)),
        }
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(return_value);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<LReturn>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FReturn {}
impl Instruction for FReturn {
    fn name(&self) -> &'static str {
        "freturn"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(FReturn {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let return_value = {
            let frame = access_macros::current_frame_mut!(thread);
            if !frame.current_method.returns_float(&frame.rt_const_pool.get_class_file())? {
                return Err(Error::IncompatibleReturnType(Opcode::FRETURN));
            }
            match frame.op_stack.pop() {
                Some(v) => Value::Float(*v.as_float()?),
                None => return Err(Error::StackUnderflow(Opcode::FRETURN)),
            }          
        };
        match thread.m_stack.pop() {
            Some(_) => (),
            None => return Err(Error::FrameStackUnderflow(Opcode::FRETURN)),
        }
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(return_value);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<FReturn>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DReturn {}
impl Instruction for DReturn {
    fn name(&self) -> &'static str {
        "dreturn"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DReturn {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let return_value = {
            let frame = access_macros::current_frame_mut!(thread);
            if !frame.current_method.returns_double(&frame.rt_const_pool.get_class_file())? {
                return Err(Error::IncompatibleReturnType(Opcode::DRETURN));
            }
            match frame.op_stack.pop() {
                Some(v) => Value::Double(*v.as_double()?),
                None => return Err(Error::StackUnderflow(Opcode::DRETURN)),
            }          
        };
        match thread.m_stack.pop() {
            Some(_) => (),
            None => return Err(Error::FrameStackUnderflow(Opcode::DRETURN)),
        }
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(return_value);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<DReturn>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AReturn {}
impl Instruction for AReturn {
    fn name(&self) -> &'static str {
        "areturn"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(AReturn {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        // This needs a lot of love
        let return_value = {
            let frame = access_macros::current_frame_mut!(thread);
            // This case needs to be fixed, see: https://docs.oracle.com/javase/specs/jls/se18/html/jls-5.html#jls-5.1.8
            if !frame.current_method.returns_reference(&frame.rt_const_pool.get_class_file())? {
                return Err(Error::IncompatibleReturnType(Opcode::ARETURN));
            }   
            let ret_val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::ARETURN)),
            };
            frame.op_stack = Vec::new();
            let current_class = Rc::clone(&frame.rt_const_pool);
            let current_class_file = current_class.get_class_file();
            {
                let ret_descriptor = frame.current_method.return_descriptor(&current_class_file)?;
                let ret_ref = ret_val.as_reference()?;
                // This is my understanding of the rules regarding reference type assignment compatibility:
                // If a type is Null, it matches with any reference type. 
                // If a type is an array, it is therefore an instance of java.lang.reflect.Array, and so can be assigned with an array or the class Object.
                // If a type is an object, it can be assigned to any interface, superinterface, or superclass.
                // Now as for Interface, I'm not too sure. I believe it just can assign to superinterfaces, but I don't know for sure. 
                
                // We actually have to write a recursive function for this situation, because of Array assignment. 
                // If we assign an array to another array, we need to check the compatibility of their inner types.
                // For now, we just allow all arrays to be cast to any other array.
                // FIXME: ^   
                match ret_ref {
                    Reference::Null => (),
                    Reference::Array(_a, _) => {
                        if !((ret_descriptor.as_bytes()[0] as char == '[') | (ret_descriptor == "java/Lang/Object")) {
                            return Err(Error::IncompatibleReturnType(Opcode::ARETURN));
                        }
                    },
                    Reference::Object(o, _) => {
                        let mut current_class = o.class().clone();
                        let mut current_class_file = current_class.get_class_file();
                        let mut found = false;
                        // These nested loops are rough, they should be tested and probably refactored.
                        while current_class_file.has_super() && !found {
                            if current_class_file.name() == ret_descriptor {
                                found = true;
                                break;
                            }
                            for interface_index in current_class_file.interfaces() {
                                let interface = current_class_file.cp_entry(*interface_index)?.as_class()?;
                                let mut current_interface = jvm.resolve_class_reference(current_class_file.cp_entry(*interface)?.as_utf8()?)?;
                                let mut found_interface = false;
                                while current_interface.get_class_file().has_super() {
                                    if current_interface.get_class_file().name() == ret_descriptor {
                                        found_interface = true;
                                        break;
                                    }
                                    current_interface = jvm.resolve_class_reference(current_interface.get_class_file().super_name().unwrap())?;
                                }
                                if found_interface {
                                    found = true;
                                    break;
                                }
                            }
                            current_class = jvm.resolve_class_reference(current_class_file.super_name().unwrap())?;
                            current_class_file = current_class.get_class_file();
                        }
                        if !found {
                            return Err(Error::IncompatibleReturnType(Opcode::ARETURN));
                        }
                    },
                    Reference::Interface(i, _) => {
                        let mut current_interface = i.clone();
                        let mut found = false;
                        // These nested loops are rough, they should be tested and probably refactored.
                        while current_interface.get_class_file().has_super() && !found {
                            if current_interface.get_class_file().name() == ret_descriptor {
                                found = true;
                                break;
                            }
                            current_interface = jvm.resolve_class_reference(current_interface.get_class_file().super_name().unwrap())?;
                        }
                        if !found {
                            return Err(Error::IncompatibleReturnType(Opcode::ARETURN));
                        }
                    }
                }
            } 
            ret_val   
        };
        let thread = access_macros::current_thread_mut!(jvm);
        match thread.m_stack.pop() {
            Some(_) => (),
            None => return Err(Error::FrameStackUnderflow(Opcode::ARETURN)),
        }
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(return_value);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<AReturn>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Return {}
impl Instruction for Return {
    fn name(&self) -> &'static str {
        "return"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(Return {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        {
            let frame = access_macros::current_frame_mut!(thread);
            if !frame.current_method.returns_void(&frame.rt_const_pool.get_class_file())? {
                return Err(Error::IncompatibleReturnType(Opcode::RETURN));
            }
        }
        match thread.m_stack.pop() {
            Some(_) => Ok(()),
            None => Err(Error::FrameStackUnderflow(Opcode::RETURN)),
        }   
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Return>() {
            None => false,
            Some(other) => self == other,
        }
    }
}




