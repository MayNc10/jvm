use std::rc::Rc;
use std::vec::Vec;

use crate::class::{Class, classfile::MethodInfo};
use crate::reference::object::Object;
use crate::value::{Value, VarValue};

#[derive(Clone)]
pub struct Frame {
    //pub data: DataContainer,
    pub local_variables: Vec<VarValue<dyn Class, dyn Object>>,
    pub op_stack: Vec<Value<dyn Class, dyn Object>>,
    pub rt_const_pool: Rc<dyn Class>, 
    pub current_method: MethodInfo, // It should be fine for this to be a straight-up methodinfo, because methodinfos should be immutable.
    pub pc: usize,
}
impl Frame {
    pub fn new(pool_ref: Rc<dyn Class>, method: MethodInfo, num_locals: usize) -> Frame {
        Frame::new_with_stack_size(pool_ref, method, num_locals, 0)
    }
    pub fn new_with_stack_size(pool_ref: Rc<dyn Class>, method: MethodInfo, num_locals: usize, stack_size: usize) -> Frame {
        let local_variables = vec![VarValue::Uninit; num_locals];
        Frame { local_variables, op_stack: Vec::with_capacity(stack_size), 
            rt_const_pool: pool_ref, current_method: method, pc: 0 }
    }

    // We should replace this call, it does nothing now
    #[inline] pub fn insert_local(&mut self, val: VarValue<dyn Class, dyn Object>, idx: usize) {   
            self.local_variables[idx] = val;
    }
}
