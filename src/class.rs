use std::{rc::Rc, any::Any};

use crate::{jvm::JVM, errorcodes::Error, value::Value, reference::object::Object};

use self::classfile::{MethodInfo, ClassFile};

pub mod classfile;
pub mod customclass;

pub trait Class {
    fn new(jvm: &mut JVM, file: classfile::ClassFile) -> Result<Self, Error> where Self : Sized;
    fn get_static(&self, name: &String, descriptor: &String, jvm: &mut JVM) -> Result<Rc<Value<dyn Class, dyn Object>>, Error>;
    fn put_static(&mut self, name: &String, descriptor: &String, value:  Value<dyn Class, dyn Object>, jvm: &mut JVM) -> Result<(), Error>;
    fn exec_method(&mut self, current_method_class: Rc<dyn Class>, jvm: &mut JVM, method: &MethodInfo) -> Result<bool, Error>; // Figure out what else to pass
    fn get_class_file(&self) -> Rc<ClassFile>;
    fn as_any(&self) ->  &dyn Any {
        self
    }
}

pub fn new_class(jvm: &mut JVM, file: classfile::ClassFile) -> Result<Rc<dyn Class>, Error> 
{
    match file.name() {
        _ => Ok(Rc::new(customclass::CustomClass::new(jvm, file)?))
    }
}