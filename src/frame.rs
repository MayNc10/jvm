use std::mem::ManuallyDrop;
use std::rc::Rc;
use std::vec::Vec;

use crate::class::{Class, classfile::MethodInfo};
use crate::reference::object::Object;
use crate::value::{Value, VarValue};
/* 
#[derive(Clone)]
pub struct DataContainer {
    pub container: Vec<Value<dyn Class, dyn Object>>,
}

impl DataContainer {
    pub fn new(num_locals: usize, stack_size: usize) -> DataContainer {
        // This is SO INCREDIBLY DANGEROUS but hopefully it's faster
        // This could be made better by merging Value and VarValue, but for now we just do this
        let mut container = Vec::with_capacity(num_locals + stack_size);
        for _ in 0..num_locals {
            let new_val = unsafe {
                std::mem::transmute::<VarValue<dyn Class, dyn Object>, 
                    Value<dyn Class, dyn Object>>(VarValue::Uninit)
            };
            container.push(new_val);
        }
        DataContainer { container}
    }
    pub fn get_local_var(&self, idx: usize) -> VarValue<dyn Class, dyn Object> {
        unsafe {
            std::mem::transmute::<Value<dyn Class, dyn Object>, 
                VarValue<dyn Class, dyn Object>>(self.container[idx].clone())
        }
    }
    pub fn get_local_var_mut(&mut self, idx: usize) -> &mut VarValue<dyn Class, dyn Object> {
        unsafe {
            std::mem::transmute::<&mut Value<dyn Class, dyn Object>, 
                &mut VarValue<dyn Class, dyn Object>>(&mut self.container[idx])
        }
    }
    pub fn set_local_var(&mut self, idx: usize, var: VarValue<dyn Class, dyn Object>) {
        let new_val = unsafe {
            std::mem::transmute::<VarValue<dyn Class, dyn Object>, 
                Value<dyn Class, dyn Object>>(var)
        };
        self.container[idx] = new_val;
    }
}
*/

#[derive(Clone)]
pub struct Frame {
    //pub data: DataContainer,
    pub local_variables: ManuallyDrop<Vec<VarValue<dyn Class, dyn Object>>>,
    pub op_stack: ManuallyDrop<Vec<Value<dyn Class, dyn Object>>>,
    pub rt_const_pool: Rc<dyn Class>, 
    pub current_method: MethodInfo, // It should be fine for this to be a straight-up methodinfo, because methodinfos should be immutable.
    // This could be made into an Rc at some point, but this *should* work for now.
    pub pc: usize,
}
impl Frame {
    pub fn new(pool_ref: Rc<dyn Class>, method: MethodInfo, num_locals: usize) -> Frame {
        Frame::new_with_stack_size(pool_ref, method, num_locals, 0)
    }
    pub fn new_with_stack_size(pool_ref: Rc<dyn Class>, method: MethodInfo, num_locals: usize, stack_size: usize) -> Frame {
        let mut v: Vec<VarValue<dyn Class, dyn Object>> = Vec::with_capacity(num_locals + stack_size);
        for _ in 0..num_locals {
            v.push(VarValue::Uninit);
        }
        let (local_variables, op_stack) = unsafe { 
            let (p1, _, _) = v.into_raw_parts();
            let p2 = p1.add(num_locals) as *mut Value<dyn Class, dyn Object>;
            (Vec::from_raw_parts(p1, num_locals, num_locals), Vec::from_raw_parts(p2, 0, stack_size))
        };

        Frame { local_variables: ManuallyDrop::new(local_variables), op_stack: ManuallyDrop::new(op_stack), 
            rt_const_pool: pool_ref, current_method: method, pc: 0 }
    }

    // We should replace this call, it does nothing now
    #[inline] pub fn insert_local(&mut self, val: VarValue<dyn Class, dyn Object>, idx: usize) {   
            self.local_variables[idx] = val;
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        let _v = unsafe {
            Vec::from_raw_parts(
                self.local_variables.as_mut_ptr(),
                self.local_variables.len() + self.op_stack.len(),
                self.local_variables.capacity() + self.op_stack.capacity(),
            )
        };
    }
}