use std::any::Any;
use std::ptr::NonNull;
use std::rc::Rc;
use std::result::Result;
use std::collections::{HashMap, hash_map};

use crate::class::{Class, classfile::*};
use crate::constant_pool::NameAndType;
use crate::errorcodes::{Error, Opcode};
use crate::jvm::JVM;
use super::object::Object;
use crate::value::{Value, ValueMarker};

// Allows for future work to remove HashMap access
struct Shape {
    mem: Box<u8>,
    layout: HashMap<NameAndType, (usize, ValueMarker)>
}

impl Shape {
    pub fn new(map: &HashMap<NameAndType, Value<dyn Class, dyn Object>>) -> Shape {

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
        let class_ref = *cm_class_file.cp_entry(field_ref.class_index)?.as_class()?;
        let class_name = cm_class_file.cp_entry(class_ref)?.as_utf8()?;
        #[allow(clippy::vtable_address_comparisons)]
        if Rc::ptr_eq(&Rc::clone(&self.class).as_dyn_rc(), &jvm.resolve_class_reference(class_name)?) {
            return Err(Error::IncompatibleObjectAndField(String::from(self.class.get_class_file().name()), String::from(class_name)));
        }
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
        let class_ref = *cm_class_file.cp_entry(field_ref.class_index)?.as_class()?;
        let class_name = cm_class_file.cp_entry(class_ref)?.as_utf8()?;
        #[allow(clippy::vtable_address_comparisons)]
        if Rc::ptr_eq(&Rc::clone(&self.class).as_dyn_rc(), &jvm.resolve_class_reference(class_name)?) {
            return Err(Error::IncompatibleObjectAndField(String::from(self.class.get_class_file().name()), String::from(class_name)));
        }
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
    fn exec_method(&mut self, _current_method_class: Rc<dyn Class>, _jvm: &mut JVM, _method: &MethodInfo) -> Result<bool, Error> {
        Err(Error::Todo(Opcode::NativeMethod))
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
