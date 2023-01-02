use std::any::Any;
use std::rc::Rc;
use std::result::Result;
use std::collections::HashMap;

use crate::class::{Class, classfile::*};
use crate::constant_pool::NameAndType;
use crate::errorcodes::{Error, Opcode};
use crate::frame::Frame;
use crate::jvm::JVM;
use crate::multitypebox::MultiTypeBox;
use crate::reference::Reference;
use crate::reference::array::Array;
use super::object::Object;
use crate::llvm::valuemarker::ValueMarker;
use crate::value::{Value, VarValue};

// Allows for future work to remove HashMap access
// TODO: Store NameAndTypes more efficiently (i.e. without strings).
struct Shape {
    mem: MultiTypeBox,
    layout: HashMap<NameAndType, (usize, ValueMarker)>
}

impl Shape {
    pub fn new(mut map: Vec<(NameAndType, Value<dyn Class, dyn Object>)>) -> Shape {
        let mut map: Vec<(&mut NameAndType, ValueMarker)> = map.iter_mut().
            map(|(n, v) |  (n, ValueMarker::from(v).unwrap())).collect();
        map.sort_by(|(_, v), (_, other)| 
            other.cmp(&v)); // Reversed to sort biggest -> smallest
        let mut counts = [0; 5];
        // count number of each
        for (_, val) in &map {
            counts[MultiTypeBox::offset_idx_from_size(val.size() as usize).unwrap()] += 1;
        }
        let mem = MultiTypeBox::new(counts).unwrap();
        let mut layout = HashMap::new();
        let mut offsets = [0; 5];
        for (name, val) in &map {
            layout.insert((*name).clone(), (offsets[MultiTypeBox::offset_idx_from_size(val.size() as usize).unwrap()], *val));
            offsets[MultiTypeBox::offset_idx_from_size(val.size() as usize).unwrap()] += 1;
        }
        Shape { mem, layout }

    }
    pub fn get_from_NAT(&self, var: NameAndType) -> Value<dyn Class, dyn Object> {
        panic!()
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct CustomObject<C> 
where
    C: Class,
    C: ?Sized,
{
    pub class: Rc<C>,
    instance_vars: HashMap<NameAndType, Value<dyn Class, dyn Object>>,
}

impl<C: Class + ?Sized + 'static> Object for CustomObject<C> {
    fn new(current_method_class: Option<Rc<dyn Class>>, class_index: Option<u16>, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> {
        // First, resolve the reference to this class.
        let cm_class_file = current_method_class.unwrap().get_class_file();
        let class_info = cm_class_file.cp_entry(class_index.unwrap())?;
        let name_index = *class_info.as_class()?;
        let name = cm_class_file.cp_entry(name_index)?.as_utf8()?;
        fn resolve_class_fields(inner_name: &str, jvm: &mut JVM, map: &mut HashMap<NameAndType, Value<dyn Class, dyn Object>>) -> Result<(), Error> {
            // We clone the name here, meaning that the name technically could become different from the actual stored name.
            // We just have to be careful to not let this happen.
            let (should_resolve, name) = {
                let current_class = jvm.resolve_class_reference(inner_name)?;
                if current_class.get_class_file().has_super() {
                    (true, Some(String::from(current_class.get_class_file().super_name().unwrap())))
                }
                else {
                    (false, None)
                }
            };
            if should_resolve {
                resolve_class_fields(name.unwrap().as_str(), jvm, map)?;
            }
            let current_class = jvm.resolve_class_reference(inner_name)?;
            for field in current_class.get_class_file().fields() {
                let name = current_class.get_class_file().cp_entry(field.name_index)?.as_utf8()?.clone();
                let descriptor = current_class.get_class_file().cp_entry(field.descriptor_index)?.as_utf8()?.clone();
                let name_and_type = NameAndType {
                    name,
                    descriptor,
                };
                let _ = map.try_insert( name_and_type, Value::new(current_class.get_class_file().cp_entry(field.descriptor_index)?.as_utf8()?));
            }
            Ok(())
        }
        let mut map = HashMap::new();
        resolve_class_fields(name, jvm, &mut map)?;
        let final_class = jvm.resolve_class_reference(name)?;
        Ok(Rc::new(CustomObject {
            class: final_class,
            instance_vars: map,
        }))
    }
    fn new_with_name(name: &str, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> {
        // W resolve the instance variables of the class.
        // This resolution is recursive, because for each class we also have to resolve the fields of its superclasses.
        // To do this, we use this helper function.
        // This function has to use mutable references instead of return values because HashMap has no append function.
        fn resolve_class_fields(inner_name: &str, jvm: &mut JVM, map: &mut HashMap<NameAndType, Value<dyn Class, dyn Object>>) -> Result<(), Error> {
            // We clone the name here, meaning that the name technically could become different from the actual stored name.
            // We just have to be careful to not let this happen.
            let (should_resolve, name) = {
                let current_class = jvm.resolve_class_reference(inner_name)?;
                if current_class.get_class_file().has_super() {
                    (true, Some(String::from(current_class.get_class_file().super_name().unwrap())))
                }
                else {
                    (false, None)
                }
            };
            if should_resolve {
                resolve_class_fields(name.unwrap().as_str(), jvm, map)?;
            }
            let current_class = jvm.resolve_class_reference(inner_name)?;
            let current_class_file = current_class.get_class_file();
            for field in current_class.get_class_file().fields().clone() {
                let new_val = {                    
                    let desc = current_class_file.cp_entry(field.descriptor_index)?.as_utf8()?;
                    Value::new(desc.as_str())
                };
                let name = current_class_file.cp_entry(field.name_index)?.as_utf8()?.clone();
                let descriptor = current_class_file.cp_entry(field.descriptor_index)?.as_utf8()?.clone();
                let name_and_type = NameAndType {
                    name,
                    descriptor,
                };
                let _ = map.try_insert( name_and_type, new_val);
            }
            Ok(())
        }
        let mut map = HashMap::new();
        resolve_class_fields(name, jvm, &mut map)?;
        let final_class = jvm.resolve_class_reference(name)?;
        Ok(Rc::new(CustomObject {
            class: final_class,
            instance_vars: map,
        }))
    }
    fn get_field(&self, current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM) -> Result<Value<dyn Class, dyn Object>, Error> {
        let cm_class_file = current_method_class.get_class_file();
        let field_ref = cm_class_file.cp_entry(class_index)?.as_field_ref()?;
        // This code is for verifing that the class the field is for is the same as this class. 
        /*
        let class_ref = *cm_class_file.cp_entry(field_ref.class_index)?.as_class()?;
        let class_name = cm_class_file.cp_entry(class_ref)?.as_utf8()?;
        #[allow(clippy::vtable_address_comparisons)]
        if !Rc::ptr_eq(&Rc::clone(&self.class).as_dyn_rc(), &jvm.resolve_class_reference(class_name)?) {
            println!("Class refs were not the same! this class pointer: {:p}, other class pointer: {:p}", 
                Rc::as_ptr(&Rc::clone(&self.class).as_dyn_rc()),  Rc::as_ptr(&jvm.resolve_class_reference(class_name)?));

            return Err(Error::IncompatibleObjectAndField(String::from(self.class.get_class_file().name()), String::from(class_name)));
        }
        */
        let name_and_type_info = cm_class_file.cp_entry(field_ref.name_and_type_index)?.as_name_and_type()?;
        let name = cm_class_file.cp_entry(name_and_type_info.name_index)?.as_utf8()?.clone();
        let descriptor = cm_class_file.cp_entry(name_and_type_info.descriptor_index)?.as_utf8()?.clone();
        let name_and_type = NameAndType {
            name,
            descriptor,
        };
        match self.instance_vars.get(&name_and_type) {
            Some(value) => {
                match value.is_reference() {
                    false => Ok(value.clone()), // If the value is a primitive type, we clone it.
                    true => Ok(Value::Reference(value.as_reference().unwrap().clone())), // If it's a reference, return a new reference. 
                    // We intentionally use '.unwrap()', because we want to crash if .is_reference() is not working correctly.
                }
            },
            None => Err(Error::NoSuchFieldError(Opcode::GETFIELD)),
        }
    }
    fn put_field(&mut self, current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM, value: Value<dyn Class, dyn Object>) -> Result<(), Error> {
        let cm_class_file = current_method_class.get_class_file();
        let field_ref = cm_class_file.cp_entry(class_index)?.as_field_ref()?;
        // This code is for verifing that the class the field is for is the same as this class. 
        /*
        let class_ref = *cm_class_file.cp_entry(field_ref.class_index)?.as_class()?;
        let class_name = cm_class_file.cp_entry(class_ref)?.as_utf8()?;
        #[allow(clippy::vtable_address_comparisons)]
        if !Rc::ptr_eq(&Rc::clone(&self.class).as_dyn_rc(), &jvm.resolve_class_reference(class_name)?) {
            println!("Class refs were not the same! this class pointer: {:p}, other class pointer: {:p}", 
                Rc::as_ptr(&Rc::clone(&self.class).as_dyn_rc()),  Rc::as_ptr(&jvm.resolve_class_reference(class_name)?));

            return Err(Error::IncompatibleObjectAndField(String::from(self.class.get_class_file().name()), String::from(class_name)));
        }
        */
        let name_and_type_info = cm_class_file.cp_entry(field_ref.name_and_type_index)?.as_name_and_type()?;
        let name = current_method_class.get_class_file().cp_entry(name_and_type_info.name_index)?.as_utf8()?.clone();
        let descriptor = current_method_class.get_class_file().cp_entry(name_and_type_info.descriptor_index)?.as_utf8()?.clone();
        let name_and_type = NameAndType {
            name,
            descriptor,
        };
        match self.instance_vars.insert(name_and_type, value) {
            Some(_) => Ok(()),
            None => Err(Error::NoSuchFieldError(Opcode::PUTFIELD)),
        }
    }
    fn exec_method(&mut self, new_method_class: Rc<dyn Class>, jvm: &mut JVM, method: &MethodInfo) 
    -> Result<bool, Error> {
        let thread = current_thread_mut!(jvm);
        // Fill out the local variables.
        let c_file = new_method_class.get_class_file();
        // Use jvm::parse_descriptor
        let (local_types, _, real_max_locals) = JVM::parse_descriptor(c_file.cp_entry(method.descriptor_index)?.as_utf8()?)?;
        let mut new_frame = Frame::new_with_stack_size(new_method_class.as_dyn_rc(), method.clone(), 
            method.code.as_ref().unwrap().max_locals.into(),
            method.code.as_ref().unwrap().max_stack.into()); // Should take into account implicit 'this'.
        let locals = &mut new_frame.local_variables; 
        // Account for implicit 'this'
       
        let mut locals_idx = real_max_locals ;
        for val in local_types.iter().rev() {
            match val {
                ValueMarker::Byte => {
                    let current_frame = current_frame_mut!(thread);
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_int(val)?;
                    locals[locals_idx] = VarValue::Byte(inner_value);
                },
                ValueMarker::Char => {
                    let current_frame = current_frame_mut!(thread);
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_int(val)?;
                    locals[locals_idx] = VarValue::Char(inner_value);
                },
                ValueMarker::Double => {
                    let current_frame = current_frame_mut!(thread);
                    locals[locals_idx] = VarValue::DoubleHighBytes;
                    locals_idx -= 1;
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_double(val)?;
                    locals[locals_idx] = VarValue::Double(inner_value);
                },
                ValueMarker::Float => {
                    let current_frame = current_frame_mut!(thread);
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_float(val)?;
                    locals[locals_idx] = VarValue::Float(inner_value);
                },
                ValueMarker::Int => {
                    let current_frame = current_frame_mut!(thread);
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_int(val)?;
                    locals[locals_idx] = VarValue::Int(inner_value);
                    
                },
                ValueMarker::Long => {
                    let current_frame = current_frame_mut!(thread);
                    locals[locals_idx] = VarValue::LongHighBytes;
                    locals_idx -= 1;
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_long(val)?;
                    locals[locals_idx] = VarValue::Long(inner_value);
                },
                ValueMarker::Short => {
                    let current_frame = current_frame_mut!(thread);
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_int(val)?;
                    locals[locals_idx] = VarValue::Short(inner_value);
                },
                ValueMarker::Reference => {            
                    let val = {
                        let current_frame = current_frame_mut!(thread);
                        // We could check the class type and make sure it matches up with the expected type, but that's not required by the JVM Spec, so for now we won't
                        match current_frame.op_stack.pop() {
                            Some(v) => v,
                            None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                        }
                    };
                    let inner_value = Value::to_reference(val)?;
                    locals[locals_idx] = VarValue::Reference(inner_value);
                }
                ValueMarker::Void => {
                    panic!("There shouldn't be void types in arguments");
                }
                ValueMarker::Top => {
                    panic!("We shouldn't read top type");
                },
            }
            locals_idx -= 1;
        }

       
        let current_frame = current_frame_mut!(thread);
        let objectref = match current_frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
        };
        let inner_ref = Value::to_reference(objectref)?;
        locals[0] = VarValue::Reference(inner_ref);

        thread.push_frame(new_frame);
        Ok(false)
    }
    fn class(&self) -> Rc<dyn Class> {
       Rc::clone(&self.class).as_dyn_rc() 
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn into_any_rc(self: Rc<Self>) -> Rc<dyn Object> {
        self
    }
    fn is_equal(&self, other: &dyn Object) -> bool {
        match other.as_any().downcast_ref::<CustomObject<C>>() {
            None => false,
            Some(other) => self.instance_vars == other.instance_vars && Rc::ptr_eq(&self.class, &other.class) ,
        }
    }
}

impl CustomObject<dyn Class> {

}
