use std::rc::Rc;
use std::result::Result;
use std::collections::HashMap;

use crate::access_macros;
use crate::class::classfile::ClassFile;
use crate::class::{Class, classfile::*};
use crate::constant_pool::{NameAndTypeInfo, NameAndType};
use crate::errorcodes::{Error, Opcode};
use crate::jvm::JVM;
use super::object::Object;
use crate::value::Value;

#[derive(Clone, PartialEq, Debug)]
pub struct CustomObject<C> 
where
    C: Class,
    C: ?Sized,
{
    pub class: Rc<C>,
    instance_vars: HashMap<NameAndType, Value<dyn Class, dyn Object>>,
}

impl<C: Class + ?Sized> Object for CustomObject<C> {
    fn new(current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM) -> Result<CustomObject<C>, Error> {
        // First, resolve the reference to this class.
        let class_info = current_method_class.cp_entry(class_index)?;
        let name_index = *class_info.as_class()?;
        let name = current_method_class.cp_entry(name_index)?.as_utf8()?;
        fn resolve_class_fields<'c ,'d, 'e>(inner_name: &str, jvm: &mut JVM, map: &mut HashMap<NameAndType, Value<dyn Class, dyn Object>>) -> Result<(), Error> {
            // We clone the name here, meaning that the name technically could become different from the actual stored name.
            // We just have to be careful to not let this happen.
            let (should_resolve, name) = {
                let current_class = jvm.resolve_class_reference(inner_name)?;
                if current_class.has_super() {
                    (true, Some(String::from(current_class.super_name().unwrap())))
                }
                else {
                    (false, None)
                }
            };
            if should_resolve {
                resolve_class_fields(name.unwrap().as_str(), jvm, map)?;
            }
            let current_class = jvm.resolve_class_reference(inner_name)?;
            for field in current_class.fields() {
                let name = current_class.cp_entry(field.name_index)?.as_utf8()?.clone();
                let descriptor = current_class.cp_entry(field.descriptor_index)?.as_utf8()?.clone();
                let name_and_type = NameAndType {
                    name,
                    descriptor,
                };
                let _ = map.try_insert( name_and_type, Value::new(current_class.cp_entry(field.descriptor_index)?.as_utf8()?));
            }
            Ok(())
        }
        let mut map = HashMap::new();
        resolve_class_fields(name, jvm, &mut map)?;
        let final_class = jvm.resolve_class_reference(name)?;
        Ok(CustomObject {
            class: final_class,
            instance_vars: map,
        })
    }
    fn new_with_name(name: &str, jvm: &mut JVM) -> Result<CustomObject<dyn Class>, Error> {
        // Now, we resolve the instance variables of the class.
        // This resolution is recursive, because for each class we also have to resolve the fields of its superclasses.
        // To do this, we use this helper function.
        // This function has to use mutable references instead of return values because HashMap has no append function.
        fn resolve_class_fields<'b>(inner_name: &str, jvm: &mut JVM, map: &mut HashMap<NameAndType, Value<dyn Class, dyn Object>>) -> Result<(), Error> {
            // We clone the name here, meaning that the name technically could become different from the actual stored name.
            // We just have to be careful to not let this happen.
            let (should_resolve, name) = {
                let current_class = jvm.resolve_class_reference(inner_name)?;
                if current_class.has_super() {
                    (true, Some(String::from(current_class.super_name().unwrap())))
                }
                else {
                    (false, None)
                }
            };
            if should_resolve {
                resolve_class_fields(name.unwrap().as_str(), jvm, map)?;
            }
            let current_class = jvm.resolve_class_reference(inner_name)?;
            for field in current_class.fields().clone() {
                let new_val = {                    
                    let desc = current_class.cp_entry(field.descriptor_index)?.as_utf8()?;
                    Value::new(desc.as_str())
                };
                let name = current_class.cp_entry(field.name_index)?.as_utf8()?.clone();
                let descriptor = current_class.cp_entry(field.descriptor_index)?.as_utf8()?.clone();
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
        Ok(CustomObject {
            class: final_class,
            instance_vars: map,
        })
    }
    fn get_field(&self, current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM) -> Result<Value<dyn Class, dyn Object>, Error> {
        let field_ref = current_method_class.cp_entry(class_index)?.as_field_ref()?;
        // This code is for verifing that the class the field is for is the same as this class. 
        let class_ref = *current_method_class.cp_entry(field_ref.class_index)?.as_class()?;
        let class_name = current_method_class.cp_entry(class_ref)?.as_utf8()?;
        if *self.m_class != *jvm.resolve_class_reference(class_name)? {
            return Err(Error::IncompatibleObjectAndField(String::from(self.m_class.name()), String::from(class_name)));
        }
        let name_and_type_info = current_method_class.cp_entry(field_ref.name_and_type_index)?.as_name_and_type()?;
        let name = current_method_class.cp_entry(name_and_type_info.name_index)?.as_utf8()?.clone();
        let descriptor = current_method_class.cp_entry(name_and_type_info.descriptor_index)?.as_utf8()?.clone();
        let name_and_type = NameAndType {
            name,
            descriptor,
        };
        match self.m_instance_vars.get(&name_and_type) {
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
        let field_ref = current_method_class.get_class_file().cp_entry(class_index)?.as_field_ref()?;
        // This code is for verifing that the class the field is for is the same as this class. 
        let class_ref = *current_method_class.get_class_file().cp_entry(field_ref.class_index)?.as_class()?;
        let class_name = current_method_class.get_class_file().cp_entry(class_ref)?.as_utf8()?;
        if *self.class != *jvm.resolve_class_reference(class_name)? {
            return Err(Error::IncompatibleObjectAndField(String::from(self.class.get_class_file().name()), String::from(class_name)));
        }
        let name_and_type_info = current_method_class.get_class_file().cp_entry(field_ref.name_and_type_index)?.as_name_and_type()?;
        let name = current_method_class.get_class_file().cp_entry(name_and_type_info.name_index)?.as_utf8()?.clone();
        let descriptor = current_method_class.get_class_file().cp_entry(name_and_type_info.descriptor_index)?.as_utf8()?.clone();
        let name_and_type = NameAndType {
            name,
            descriptor,
        };
        match self.instance_vars.insert(name_and_type, value) {
            Some(_) => Ok(()),
            None => Err(Error::NoSuchFieldError(Opcode::GETFIELD)),
        }
    }
    fn exec_method(&mut self, current_method_class: Rc<dyn Class>, jvm: &mut JVM, method: &MethodInfo) -> Result<bool, Error> {
        todo!("exec method customclass")
    }
    fn class(&self) -> Rc<C> {
        self.class.clone()
    }
}

