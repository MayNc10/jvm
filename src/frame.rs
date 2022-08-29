use std::rc::Rc;
use std::vec::Vec;

use crate::class::{Class, MethodInfo};
use crate::value::{Value, VarValue};

#[derive(Clone)]
pub struct Frame {
    pub local_variables: Vec<VarValue>,
    pub op_stack: Vec< Value>,
    pub rt_const_pool: Rc< Class>, 
    pub current_method: MethodInfo, // It should be fine for this to be a straight-up methodinfo, because methodinfos should be immutable.
    // This could be made into an Rc at some point, but this *should* work for now.
}
impl Frame {
    pub fn new(pool_ref: Rc< Class>, method: MethodInfo) -> Frame {
        Frame { local_variables: Vec::new(), op_stack: Vec::new(), rt_const_pool: pool_ref, current_method:  method}
    }
    
     
}