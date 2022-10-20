use crate::{class::customclass::CustomClass, errorcodes::Opcode, frame::Frame, access_macros};

use super::super::*;

pub struct String {
    s: std::string::String,
    s_class: Rc<dyn Class>, 
}

impl Object for String {
    fn new(current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self: Sized {
        Ok(Rc::new(String {s: std::string::String::from(""), 
        s_class: jvm.resolve_class_reference("java/lang/String")?}))
    }
    fn new_with_name(name: &str, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self: Sized {
        panic!("No one should ever call String::new_with_name")
    }
    fn get_field(&self, current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM) -> Result<Value<dyn Class, dyn Object>, Error> {
        Err(Error::NoSuchFieldError(Opcode::GETFIELD))
    }
    fn put_field(&mut self, current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM, value: Value<dyn Class, dyn Object>) -> Result<(), Error> {
        Err(Error::NoSuchFieldError(Opcode::PUTFIELD))
    }
    fn exec_method(&mut self, current_method_class: Rc<dyn Class>, jvm: &mut JVM, method: &MethodInfo) -> Result<bool, Error> {
        let name = current_method_class.get_class_file().cp_entry(method.name_index)?.as_utf8()?.as_str();
        let desc = current_method_class.get_class_file().cp_entry(method.descriptor_index)?.as_utf8()?.as_str();
        let thread = access_macros::current_thread_mut!(jvm);
        let frame: &mut Frame = access_macros::current_frame_mut!(thread);
        let mut was_natively_executed = true;
        match (name, desc) {
            ("<init>", "()V") => {
                // Nothing to do, because we already init with an empty string.
            },
            ("<init>", "([B)V") => {
                let aval = match frame.op_stack.pop() {
                    Some(Value::Reference(r)) => r,
                    Some(_) => return Err(Error::UnexpectedTypeOnStack(Opcode::MethodInvoke)),
                    None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                };
                let aref = match aval {
                    Reference::Array(a, _) => a,
                    _ => return Err(Error::IncorrectReferenceType(Opcode::MethodInvoke)),
                };
                let array = unsafe { Rc::get_mut_unchecked(&mut aref) };
                //if !array.bytea
            },
            _ => {
                // do funky stuff
                was_natively_executed = false;
            }
        }
        Ok(was_natively_executed)
    }
    fn class(&self) -> Rc<dyn Class> {
        self.s_class.clone()
    }
    fn into_any_rc(self: Rc<Self>) -> Rc<dyn Object> {
        self
    }

}