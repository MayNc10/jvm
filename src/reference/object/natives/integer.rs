use crate::{access_macros, frame::Frame, thread::Thread, errorcodes::Opcode, class::customclass::CustomClass};

use super::super::*;
struct Integer {
    i: i32,
    // this should work, could be a param type if needed.
    int_class: Rc<dyn Class>,
} 

impl Object for Integer {
    fn new(current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self: Sized {
        Ok(Rc::new(Integer {i: 0, int_class: jvm.resolve_class_reference("java/lang/Integer")?}))
    }
    fn new_with_name(name: &str, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error>  where Self : Sized {
        panic!("No one should ever call Integer::new_with_name")
    }
    fn get_field(&self, current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM) -> Result<Value<dyn Class, dyn Object>, Error> {
        Err(Error::NoSuchFieldError(Opcode::GETFIELD))
    }
    fn put_field(&mut self, current_method_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM, value: Value<dyn Class, dyn Object>) -> Result<(), Error> {
        Err(Error::NoSuchFieldError(Opcode::PUTFIELD))
    }
    fn exec_method(&mut self, current_method_class: Rc<dyn Class>, jvm: &mut JVM, method: &MethodInfo) -> Result<bool, Error> {
        let cm_class_file = current_method_class.get_class_file();
        let name = cm_class_file.cp_entry(method.name_index)?.as_utf8()?;
        let desc = cm_class_file.cp_entry(method.descriptor_index)?.as_utf8()?;
        let thread = access_macros::current_thread_mut!(jvm);
        let frame: &mut Frame = access_macros::current_frame_mut!(thread);
        let mut was_natively_executed = true;
        match (name.as_str(), desc.as_str()) {
            ("<init>", "(I)V") => {
                let i = match frame.op_stack.pop() {
                    Some(Value::Int(i)) => i,
                    Some(_) => return Err(Error::UnexpectedTypeOnStack(Opcode::MethodInvoke)),
                    None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                };
                self.i = i;
            },
            ("<init>", "(Ljava/lang/String;)V") => {
                let sref = match frame.op_stack.pop() {
                    Some(Value::Reference(r)) => r,
                    Some(_) => return Err(Error::UnexpectedTypeOnStack(Opcode::MethodInvoke)),
                    None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                };
                let sobj: Rc<dyn Object> = match sref {
                    Reference::Object(obj, _) => obj,
                    _ => return Err(Error::IncorrectReferenceType(Opcode::MethodInvoke)),
                };
                let s = match sobj.as_any().downcast_ref::<super::string::String>() {
                    Some(s) => s,
                    None => return Err(Error::IncorrectReferenceType(Opcode::MethodInvoke)), // This might not be the right error.
                };
                // do some things with the string.
            },
            _ => {
                // do funky stuff
                was_natively_executed = false;
            }
        }
        Ok(was_natively_executed)
    }
    fn class(&self) -> Rc<dyn Class> {
        self.int_class.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn into_any_rc(self: Rc<Self>) -> Rc<dyn Object> {
        self
    }
    fn is_equal(&self, other: &dyn Object) -> bool {
        match other.as_any().downcast_ref::<Integer>() {
            None => false,
            Some(other) => self.i == other.i,
        }
    }
}
