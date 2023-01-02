use std::any::Any;

use super::*;
use crate::{value::Value, class::classfile::MethodInfo};

pub mod customobject;
pub mod natives;

pub trait Object {
    // Should create a new object, and init its fields with zeros.
    #[allow(clippy::new_ret_no_self)]
    fn new(current_method_class: Option<Rc<dyn Class>>, class_index: Option<u16>, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self : Sized;
    // Needed for CustomObject, but this should never be called on a native object.
    fn new_with_name(name: &str, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self : Sized {
        customobject::CustomObject::<dyn Class>::new_with_name(name, jvm)
    }
    // This gives an index into the rt const pool of the class of the object, which references a field. 
    fn get_field(&self, current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM) -> Result<Value<dyn Class, dyn Object>, Error>;
    // This gives an index into the rt const pool of the class of the object, which references a field. 
    // It also gives a value to be put in that field.
    fn put_field(&mut self, current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM, value: Value<dyn Class, dyn Object>) -> Result<(), Error>;
    // This is run after the JVM decides which function to execute. Just run the code.
    fn exec_method(&mut self, current_method_class: Rc<dyn Class>, jvm: &mut JVM, method: &MethodInfo) 
    -> Result<bool, Error>; // Figure out what else to pass
    // Give back the class file that backs this object.
    fn class(&self) -> Rc<dyn Class>;
    fn as_any(&self) -> &dyn Any;
    fn into_any_rc(self: Rc<Self>) -> Rc<dyn Object>;
    fn is_equal(&self, other: &dyn Object) -> bool;
}

pub fn new_object_with_name(name: &str, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> {
    match name {
        "java/lang/String" => natives::string::String::new(None, None, jvm),
        "java/io/PrintStream" => natives::print_stream::PrintStream::new(None, None, jvm),
        "java/lang/StringBuilder" => natives::string_builder::StringBuilder::new(None, None, jvm),
        _ => customobject::CustomObject::<dyn Class>::new_with_name(name, jvm)
    }
}

pub fn new_object(current_method_class: Option<Rc<dyn Class>>, class_index: Option<u16>, jvm: &mut JVM) 
-> Result<Rc<dyn Object>, Error> {
    // First, resolve the reference to this class.
    let cm_class_file = current_method_class.unwrap().get_class_file();
    let class_info = cm_class_file.cp_entry(class_index.unwrap())?;
    let name_index = *class_info.as_class()?;
    let name = cm_class_file.cp_entry(name_index)?.as_utf8()?;
    new_object_with_name(name.as_str(), jvm)

}

impl PartialEq for dyn Object {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}