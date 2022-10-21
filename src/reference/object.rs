use std::any::Any;

use self::customobject::CustomObject;

use super::*;
use crate::{value::Value, class::classfile::MethodInfo};

pub mod customobject;
pub mod natives;

pub trait Object {
    // Should create a new object, and init its fields with zeros.
    fn new(current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self : Sized;
    // Needed for CustomObject, but this should never be called on a native object.
    fn new_with_name(name: &str, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self : Sized {
        match name {
            _ => customobject::CustomObject::<dyn Class>::new_with_name(name, jvm)
        }
    }
    // This gives an index into the rt const pool of the class of the object, which references a field. 
    fn get_field(&self, current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM) -> Result<Value<dyn Class, dyn Object>, Error>;
    // This gives an index into the rt const pool of the class of the object, which references a field. 
    // It also gives a value to be put in that field.
    fn put_field(&mut self, current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM, value: Value<dyn Class, dyn Object>) -> Result<(), Error>;
    // This is run after the JVM decides which function to execute. Just run the code.
    fn exec_method(&mut self, current_method_class: Rc<dyn Class>, jvm: &mut JVM, method: &MethodInfo) -> Result<bool, Error>; // Figure out what else to pass
    // Give back the class file that backs this object.
    fn class(&self) -> Rc<dyn Class>;
    fn as_any(&self) -> &dyn Any {
        &self
    }
    fn into_any_rc(self: Rc<Self>) -> Rc<dyn Object>;
    fn is_equal(&self, other: &dyn Object) -> bool;
}

impl PartialEq for dyn Object {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}