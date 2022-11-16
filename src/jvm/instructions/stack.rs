use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Pop {}
impl Instruction for Pop {
    fn name(&self) -> &'static str {
        "pop"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(Pop {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        match frame.op_stack.pop() {
            Some(val) => {
                if val.is_comptype2() {
                    return Err(Error::IncorrectComputationalType(Opcode::POP));
                }
            },
            None => return Err(Error::StackUnderflow(Opcode::POP)),
        }
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
pub struct Pop2 {}
impl Instruction for Pop2 {
    fn name(&self) -> &'static str {
        "pop2"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(Pop2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        match frame.op_stack.pop() {
            Some(val) => {
                if val.is_comptype2() {
                    return Ok(());
                }
                else {
                    match frame.op_stack.pop() {
                        Some(val) => {
                            if val.is_comptype2() {
                                return Err(Error::IncorrectComputationalType(Opcode::POP2));
                            }
                        },
                        None => return Err(Error::StackUnderflow(Opcode::POP2)),
                    }
                }
            },
            None => return Err(Error::StackUnderflow(Opcode::POP2)),
        }
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
pub struct Dup {}
impl Instruction for Dup {
    fn name(&self) -> &'static str {
        "dup"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(Dup {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let top_index = frame.op_stack.len();
        if top_index == 0 {
            return Err(Error::StackUnderflow(Opcode::DUP));
        }
        let val = frame.op_stack[top_index - 1].clone();
        if val.is_comptype2() {
            return Err(Error::IncorrectComputationalType(Opcode::DUP));
        }
        frame.op_stack.push(val);
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
pub struct DupX1 {}
impl Instruction for DupX1 {
    fn name(&self) -> &'static str {
        "dup_x1"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DupX1 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUPX1)),
        };
        if val.is_comptype2() {
            return Err(Error::IncorrectComputationalType(Opcode::DUPX1));
        }
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUPX1)),
        };
        if val2.is_comptype2() {
            return Err(Error::IncorrectComputationalType(Opcode::DUPX1));
        }
        frame.op_stack.push(val.clone());
        frame.op_stack.push(val2);
        frame.op_stack.push(val);
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
pub struct DupX2 {}
impl Instruction for DupX2 {
    fn name(&self) -> &'static str {
        "dup_x2"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(DupX2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUPX2)),
        };
        if val.is_comptype2() {
            return Err(Error::IncorrectComputationalType(Opcode::DUPX2));
        }
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUPX2)),
        };
        if val2.is_comptype1() {
            let val3 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::DUPX2)),
            };
            if val3.is_comptype2() {
                return Err(Error::IncorrectComputationalType(Opcode::DUPX2));
            }
            frame.op_stack.push(val.clone());
            frame.op_stack.push(val3);
            frame.op_stack.push(val2);
            frame.op_stack.push(val);
        }
        else {
            frame.op_stack.push(val.clone());
            frame.op_stack.push(val2);
            frame.op_stack.push(val);
        }
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
pub struct Dup2 {}
impl Instruction for Dup2 {
    fn name(&self) -> &'static str {
        "dup2"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(Dup2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let top_index = frame.op_stack.len();
        if top_index == 0 {
            return Err(Error::StackUnderflow(Opcode::DUP2));
        }
        let val = frame.op_stack[top_index - 1].clone();
        if val.is_comptype1() {
            if top_index == 1 {
                return Err(Error::StackUnderflow(Opcode::DUP2));
            }
            let val2 = frame.op_stack[top_index - 2].clone();
            frame.op_stack.push(val2);
            frame.op_stack.push(val);
        }
        else {
            frame.op_stack.push(val);
        }
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
pub struct Dup2X1 {}
impl Instruction for Dup2X1 {
    fn name(&self) -> &'static str {
        "dup2_x1"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(Dup2X1 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUP2X1)),
        };
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUP2X1)),
        };
        if val2.is_comptype2() {
            return Err(Error::IncorrectComputationalType(Opcode::DUP2X1));
        }
        if val.is_comptype1() {
            let val3 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::IncorrectComputationalType(Opcode::DUP2X1)),
            };
            if val3.is_comptype2() {
                return Err(Error::IncorrectComputationalType(Opcode::DUP2X1));
            }
            frame.op_stack.push(val2.clone());
            frame.op_stack.push(val.clone());
            frame.op_stack.push(val3);
            frame.op_stack.push(val2);
            frame.op_stack.push(val);
        }
        else {
            frame.op_stack.push(val.clone());
            frame.op_stack.push(val2);
            frame.op_stack.push(val);
        }
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
pub struct Dup2X2 {}
impl Instruction for Dup2X2 {
    fn name(&self) -> &'static str {
        "dup2_x2"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(Dup2X2 {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUP2X2)),
        };
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUP2X2)),
        };
        if val.is_comptype1() {
            if val2.is_comptype2() {
                return Err(Error::IncorrectComputationalType(Opcode::DUP2X2));
            }
            else {
                let val3 = match frame.op_stack.pop() {
                    Some(v) => v,
                    None => return Err(Error::StackUnderflow(Opcode::DUP2X2)),
                };
                if val3.is_comptype1() {
                    let val4 = match frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::DUP2X2)),
                    };
                    if val4.is_comptype2() {
                        return Err(Error::IncorrectComputationalType(Opcode::DUP2X2));
                    }
                    frame.op_stack.push(val2.clone());
                    frame.op_stack.push(val.clone());
                    frame.op_stack.push(val4);
                    frame.op_stack.push(val3);
                    frame.op_stack.push(val2);
                    frame.op_stack.push(val);
                }
                else {
                    frame.op_stack.push(val2.clone());
                    frame.op_stack.push(val.clone());
                    frame.op_stack.push(val3);
                    frame.op_stack.push(val2);
                    frame.op_stack.push(val); 
                }

            }
        }   
        else if val2.is_comptype1() {
                let val3 = match frame.op_stack.pop() {
                    Some(v) => v,
                    None => return Err(Error::StackUnderflow(Opcode::DUP2X2)),
                };
                if val3.is_comptype2() {
                    return Err(Error::IncorrectComputationalType(Opcode::DUP2X2));
                }
                frame.op_stack.push(val.clone());
                frame.op_stack.push(val3);
                frame.op_stack.push(val2);
                frame.op_stack.push(val);
            }
        else {
            frame.op_stack.push(val.clone());
            frame.op_stack.push(val2);
            frame.op_stack.push(val); 
        }
        
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
pub struct Swap {}
impl Instruction for Swap {
    fn name(&self) -> &'static str {
        "swap"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(Swap {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::SWAP)),
        };
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::SWAP)),
        };
        frame.op_stack.push(val);
        frame.op_stack.push(val2);
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