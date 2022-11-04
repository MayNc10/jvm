use std::{rc::Rc, any::Any};

use crate::{jvm::JVM, errorcodes::Error, value::Value, reference::object::Object};

use self::classfile::{MethodInfo, ClassFile};

pub mod classfile;
pub mod customclass;
pub mod system;
 
pub trait Class {
    fn new(file: classfile::ClassFile, jvm: &mut JVM) -> Result<Self, Error> where Self : Sized;
    fn get_static(&self, name: &str, descriptor: &str, jvm: &mut JVM) -> Result<Value<dyn Class, dyn Object>, Error>;
    fn put_static(&mut self, name: &str, descriptor: &str, value:  Value<dyn Class, dyn Object>, jvm: &mut JVM) -> Result<(), Error>;
    fn exec_method(self: Rc<Self>, jvm: &mut JVM, method: &MethodInfo) -> Result<bool, Error>; // Figure out what else to pass
    fn get_class_file(&self) -> Rc<ClassFile>;
    fn as_any(&self) ->  &dyn Any;
    fn as_any_rc(self: Rc<Self>) -> Rc<dyn Any>;
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn Class>;
}

/* 
impl PartialEq for dyn Class {
    fn eq(&self, other: &Self) -> bool {
        self.get_class_file() == other.get_class_file()-
    }
    fn ne(&self, other: &Self) -> bool {
        self.get_class_file() != other.get_class_file()
    }
}
*/

pub fn new_class(file: classfile::ClassFile, jvm: &mut JVM) -> Result<Rc<dyn Class>, Error> {
    match file.name() {
        "java/lang/System" => Ok(Rc::new(system::System::new(file, jvm)?)),
        _ => Ok(Rc::new(customclass::CustomClass::new(file, jvm)?)),
    }
}
