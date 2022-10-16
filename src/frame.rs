use std::rc::Rc;
use std::vec::Vec;

use crate::class::{Class, classfile::MethodInfo};
use crate::reference::object::Object;
use crate::value::{Value, VarValue};

#[derive(Clone)]
pub struct Frame {
    pub local_variables: Vec<VarValue<dyn Class, dyn Object>>,
    pub op_stack: Vec<Value<dyn Class, dyn Object>>,
    pub rt_const_pool: Rc<dyn Class>, 
    pub current_method: MethodInfo, // It should be fine for this to be a straight-up methodinfo, because methodinfos should be immutable.
    // This could be made into an Rc at some point, but this *should* work for now.
    pub pc: usize,
}
impl Frame {
    pub fn new(pool_ref: Rc<dyn Class>, method: MethodInfo) -> Frame {
        Frame { local_variables: Vec::new(), op_stack: Vec::new(), rt_const_pool: pool_ref, current_method: method, pc: 0}
    }
    pub fn insert_local(&mut self, val: VarValue<dyn Class, dyn Object>, idx: usize) {
        if idx < self.local_variables.len() {
            self.local_variables[idx] = val;
        }
        else {
            self.local_variables.reserve(idx + 1 - self.local_variables.len());
            for _ in 0..(idx - self.local_variables.len()) {
                self.local_variables.push(VarValue::Uninit);
            }
            self.local_variables.push(val);
        }
    }
     
}