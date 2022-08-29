use std::rc::Rc;
use std::result::Result;
use std::collections::HashMap;

use crate::access_macros;
use crate::class::Class;
use crate::constant_pool::NameAndTypeInfo;
use crate::errorcodes::{Error, Opcode};
use crate::jvm::JVM;
use crate::value::Value;

#[derive(Clone, PartialEq, Debug)]
pub struct Object {
    pub m_class: Rc<Class>,
    m_instance_vars: HashMap<NameAndTypeInfo, Value>,
}

impl<'a>  Object {
    //  This function takes the current class and jvm in order to resolve references to the class that it's an instance of.
    // This also could be implmented as a JVM-side functions. I'm not good at designing or OO, so this probably could be better designed.
    pub fn new<'b>(current_method_class: Rc<Class>, class_index: u16, loaded_classes: &'b mut HashMap<String, Rc<Class>>) -> Result< Object, Error> 
    where 
        'b : 'a,
    {
        // First, resolve the reference to this class.
        let class_info = current_method_class.cp_entry(class_index)?;
        let name_index = *class_info.as_class()?;
        let name = current_method_class.cp_entry(name_index)?.as_utf8()?;
        fn resolve_class_fields<'c ,'d, 'e>(inner_name: &str, inner_loaded_classses: &mut HashMap<String, Rc<Class>>, map: &mut HashMap<NameAndTypeInfo,  Value>) -> Result<(), Error> 
        {
            // We clone the name here, meaning that the name technically could become different from the actual stored name.
            // We just have to be careful to not let this happen.
            let (should_resolve, name) = {
                let current_class = access_macros::resolve_class_reference!(*inner_loaded_classses, inner_name)?;
                if current_class.has_super() {
                    (true, Some(String::from(current_class.super_name().unwrap())))
                }
                else {
                    (false, None)
                }
            };
            if should_resolve {
                resolve_class_fields(name.unwrap().as_str(), inner_loaded_classses, map)?;
            }
            let current_class = access_macros::resolve_class_reference!(*inner_loaded_classses, inner_name)?;
            for field in current_class.fields() {
                let _ = map.try_insert( NameAndTypeInfo{ 
                    name_index: field.name_index,
                    descriptor_index: field.descriptor_index,
                }, Value::new(current_class.cp_entry(field.descriptor_index)?.as_utf8()?));
            }
            Ok(())
        }
        let mut map = HashMap::new();
        resolve_class_fields(name, loaded_classes, &mut map)?;
        let final_class = access_macros::resolve_class_reference!(*loaded_classes, name)?;
        Ok(Object {
            m_class: final_class,
            m_instance_vars: map,
        })
    }
    pub fn new_with_name<'b, 'c>(name: &'c str, loaded_classes: &'b mut HashMap<String, Rc<Class>>) -> Result< Object, Error> 
    {
        // Now, we resolve the instance variables of the class.
        // This resolution is recursive, because for each class we also have to resolve the fields of its superclasses.
        // To do this, we use this helper function.
        // This function has to use mutable references instead of return values because HashMap has no append function.
        fn resolve_class_fields<'b>(inner_name: &str, inner_loaded_classses: &mut HashMap<String, Rc<Class>>, map: &mut HashMap<NameAndTypeInfo, Value>) -> Result<(), Error> 
        {
            // We clone the name here, meaning that the name technically could become different from the actual stored name.
            // We just have to be careful to not let this happen.
            let (should_resolve, name) = {
                let current_class = access_macros::resolve_class_reference!(*inner_loaded_classses, inner_name)?;
                if current_class.has_super() {
                    (true, Some(String::from(current_class.super_name().unwrap())))
                }
                else {
                    (false, None)
                }
            };
            if should_resolve {
                resolve_class_fields(name.unwrap().as_str(), inner_loaded_classses, map)?;
            }
            let current_class = access_macros::resolve_class_reference!(*inner_loaded_classses, inner_name)?;
            for field in current_class.fields().clone() {
                let new_val = {                    
                    let desc = current_class.cp_entry(field.descriptor_index)?.as_utf8()?;
                    Value::new(desc.as_str())
                };
                let _ = map.try_insert( NameAndTypeInfo{ 
                    name_index: field.name_index,
                    descriptor_index: field.descriptor_index,
                }, new_val);
            }
            Ok(())
        }
        let mut map = HashMap::new();
        resolve_class_fields(name, loaded_classes, &mut map)?;
        let final_class = access_macros::resolve_class_reference!(*loaded_classes, name)?;
        Ok(Object {
            m_class: final_class,
            m_instance_vars: map,
        })
    }
    pub fn get_field(&self, current_method_class: Rc<Class>, class_index: u16, loaded_classes: &mut HashMap<String, Rc<Class>>) -> Result< Value, Error> {
        let field_ref = current_method_class.cp_entry(class_index)?.as_field_ref()?;
        // This code is for verifing that the class the field is for is the same as this class. 
        let class_ref = *current_method_class.cp_entry(field_ref.class_index)?.as_class()?;
        let class_name = current_method_class.cp_entry(class_ref)?.as_utf8()?;
        if *self.m_class != *access_macros::resolve_class_reference!(*loaded_classes, class_name)? {
            return Err(Error::IncompatibleObjectAndField(String::from(self.m_class.name()), String::from(class_name)));
        }
        let name_and_type = current_method_class.cp_entry(field_ref.name_and_type_index)?.as_name_and_type()?;
        match self.m_instance_vars.get(name_and_type) {
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
    pub fn put_field<'b, 'c>(&mut self, current_method_class: Rc<Class>, class_index: u16, jvm: &'b mut JVM, value:  Value) -> Result<(), Error> 
    where
        'b : 'c,
    {
        let field_ref = current_method_class.cp_entry(class_index)?.as_field_ref()?;
        // This code is for verifing that the class the field is for is the same as this class. 
        let class_ref = *current_method_class.cp_entry(field_ref.class_index)?.as_class()?;
        let class_name = current_method_class.cp_entry(class_ref)?.as_utf8()?;
        if *self.m_class != *access_macros::resolve_class_reference!(jvm.m_loaded_classes, class_name)? {
            return Err(Error::IncompatibleObjectAndField(String::from(self.m_class.name()), String::from(class_name)));
        }
        let name_and_type = current_method_class.cp_entry(field_ref.name_and_type_index)?.as_name_and_type()?;
        match self.m_instance_vars.insert(*name_and_type, value) {
            Some(_) => Ok(()),
            None => Err(Error::NoSuchFieldError(Opcode::GETFIELD)),
        }
    }
}

