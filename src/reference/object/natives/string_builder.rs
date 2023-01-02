use crate::{errorcodes::Opcode, frame::Frame, access_macros};

use super::super::*;

pub struct StringBuilder {
    inner: String,
    s_class: Rc<dyn Class>,
}

impl Object for StringBuilder {
    fn new(_current_method_class: Option<Rc<dyn Class>>, _class_index: Option<u16>, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self: Sized {
        // Once we implement String class, we should use a hashmap and stuff here.
        Ok(Rc::new(StringBuilder { inner: String::new(),
            s_class: jvm.resolve_class_reference("java/lang/StringBuilder")? } ))
    }
    fn new_with_name(_name: &str, _jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self: Sized {
        panic!("No one should ever call StringBuilder::new_with_name")
    }
    fn get_field(&self, _current_method_class: Rc<dyn Class>, _class_index: u16, _jvm: &mut JVM) -> Result<Value<dyn Class, dyn Object>, Error> {
        Err(Error::NoSuchFieldError(Opcode::GETFIELD))
    }
    fn put_field(&mut self, _current_method_class: Rc<dyn Class>, _class_index: u16, _jvm: &mut JVM, _value: Value<dyn Class, dyn Object>) -> Result<(), Error> {
        Err(Error::NoSuchFieldError(Opcode::PUTFIELD))
    }
    fn exec_method(&mut self, current_method_class: Rc<dyn Class>, jvm: &mut JVM, method: &MethodInfo) 
    -> Result<bool, Error> {
        let cm_class_file = current_method_class.get_class_file();
        let name = cm_class_file.cp_entry(method.name_index)?.as_utf8()?.as_str();
        let desc = cm_class_file.cp_entry(method.descriptor_index)?.as_utf8()?.as_str();
        let thread = access_macros::current_thread_mut!(jvm);
        let frame: &mut Frame = access_macros::current_frame_mut!(thread);
        let mut popped_self = false;
        let mut was_natively_executed = true;
        match (name, desc) {
            ("<init>", "()V") => {},
            ("<init>", "(I)V") => {
                let capacity = frame.op_stack.pop().unwrap();
                self.inner = String::with_capacity(capacity.to_int()? as usize);
            }
            ("<init>", "(Ljava/lang/String;)V") => {
                let s_ref = frame.op_stack.pop().unwrap();
                let s_ref = s_ref.to_reference()?;
                let s = s_ref.as_object()?.as_any().downcast_ref::<natives::string::String>().unwrap();
                self.inner = s.backing_string().clone();
            }
            ("append", "(Ljava/lang/String;)Ljava/lang/StringBuilder;") => {
                let s_ref = frame.op_stack.pop().unwrap();
                let s_ref = s_ref.to_reference()?;
                let s = s_ref.as_object()?.as_any().downcast_ref::<natives::string::String>().unwrap();
                self.inner.push_str(&s.backing_string());
                let this_as_ref = frame.op_stack.pop().unwrap().to_reference()?;
                popped_self = true;
                frame.op_stack.push(Value::Reference(this_as_ref));
            },
            ("append", "(D)Ljava/lang/StringBuilder;") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                let s = if val == val.round() {
                    format!("{val:.1}")
                    
                } 
                else {
                     format!("{val}")
                };
                self.inner.push_str(s.as_str());
                let this_as_ref = frame.op_stack.pop().unwrap().to_reference()?;
                popped_self = true;
                frame.op_stack.push(Value::Reference(this_as_ref));
            },
            ("toString", "()Ljava/lang/String;") => {
                let s = natives::string::String::new_from_string(self.inner.clone(), jvm)?;
                let thread = access_macros::current_thread_mut!(jvm);
                let frame: &mut Frame = access_macros::current_frame_mut!(thread);
                let obj = Reference::Object(s as Rc<dyn Object>, Rc::new(Monitor::new()));
                let val = Value::Reference(obj);
                frame.op_stack.pop();
                popped_self = true;
                frame.op_stack.push(val);
            }
            _ => {
                // do funky stuff
                eprintln!("{}", format!("Use of unimplemented function: {name}{desc} in class StringBuilder").red());
                if &desc[desc.len() - 1..] != "V" {
                    // expected to push something onto stack
                    frame.op_stack.push(Value::Reference(Reference::Null));
                }
                was_natively_executed = false;
            }
        }
        if !popped_self {
            let thread = access_macros::current_thread_mut!(jvm);
            let frame: &mut Frame = access_macros::current_frame_mut!(thread);
            frame.op_stack.pop();
        }
        Ok(was_natively_executed)
    }
    fn class(&self) -> Rc<dyn Class> {
        self.s_class.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn into_any_rc(self: Rc<Self>) -> Rc<dyn Object> {
        self
    }
    fn is_equal(&self, other: &dyn Object) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(_other) => true,
        }
    }

}
