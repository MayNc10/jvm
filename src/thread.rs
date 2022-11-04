use std::rc::Rc;
use std::vec::Vec;

use crate::class::Class;
use crate::errorcodes::Error;
use crate::frame::Frame;
use crate::reference::Monitor;
use crate::reference::object::Object;
use crate::value::{Value, VarValue};


// FIXME: Because we access the length so much, should we just store it?

pub struct Thread {
    pub m_stack: Vec<Frame>,
    // This is a bad name, but I can't think of a better one right now.
    // This could be an unsafe mutable pointer, because we only modify if we're the only owner, and we (will) modify it atomically.
    // However, for now, this is easier. 
    pub current_monitor: Option<Rc<Monitor>>,
    pub next_instruction_is_wide: bool,
}
impl Thread {
    pub fn new() -> Thread {
        Thread {m_stack: Vec::new(), current_monitor: None, next_instruction_is_wide: false }
    }
    pub fn clone_current_frame(&mut self) -> Frame {
        self.current_frame().clone()
    }
    pub fn current_frame(&self) -> &Frame {
        let length = self.m_stack.len();
        &self.m_stack[length - 1]
    }
    pub fn current_frame_mut(&mut self) -> &mut Frame {
        let length = self.m_stack.len();
        &mut self.m_stack[length -1]
    }
    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.m_stack.pop()
    }
    pub fn replace_current_frame(&mut self, frame: Frame) {
        let length = self.m_stack.len();
        self.m_stack[length - 1] = frame;
    }
    pub fn push_frame(&mut self, frame: Frame) {
        self.m_stack.push(frame);
    }
    pub fn pc(&self) -> usize {
        self.m_stack.last().unwrap().pc
    }
    pub fn set_pc(&mut self, new_pc: usize) -> Result<(), Error> {
        let max_pc = {
            self.current_frame().current_method.code()?.len()
        };
        if new_pc > max_pc {
            return Err(Error::ProgramCounterOverflow);
        }
        self.m_stack.last_mut().unwrap().pc = new_pc;
        Ok(())
    }
    pub fn inc_pc(&mut self, added_pc: isize) -> Result<(), Error> {
        let max_pc = {
            self.current_frame().current_method.code()?.len() as isize
        };
        if self.m_stack.last().unwrap().pc as isize + added_pc > max_pc {
            return Err(Error::ProgramCounterOverflow);
        }
        self.m_stack.last_mut().unwrap().pc = (self.m_stack.last().unwrap().pc as isize + added_pc) as usize;
        Ok(())
    }
    pub fn push_op(&mut self, op:  Value<dyn Class, dyn Object>) {
        let length = self.m_stack.len();
        let op_len = self.m_stack[length - 1].op_stack.len();
        self.m_stack[length-1].op_stack[op_len-1] = op;
    } 
    pub fn pop_op(&mut self) -> Option<Value<dyn Class, dyn Object>> {
        let length = self.m_stack.len();
        self.m_stack[length - 1].op_stack.pop()
    }
    pub fn push_var(&mut self, var: VarValue<dyn Class, dyn Object>) {
        let length = self.m_stack.len();
        let var_len = self.m_stack[length - 1].local_variables.len();
        self.m_stack[length-1].local_variables[var_len-1] = var;
    }
} 

impl Default for Thread {
    fn default() -> Self {
        Self::new()
    }
}  