use crate::{errorcodes::Opcode, frame::Frame, access_macros};

use super::super::*;

pub struct String {
    s: std::string::String,
    s_class: Rc<dyn Class>, 
}

impl Object for String {
    fn new(_current_method_class: Option<Rc<dyn Class>>, _class_index: Option<u16>, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self: Sized {
        // Once we implement String class, we should use a hashmap and stuff here.
        Ok(Rc::new(String {s: std::string::String::from(""), 
        s_class: jvm.resolve_class_reference("java/lang/String")?}))
    }
    fn new_with_name(_name: &str, _jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self: Sized {
        panic!("No one should ever call String::new_with_name")
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
            ("<init>", "()V") => {
                // Nothing to do, because we already init with an empty string.
            },
            ("length", "()I") => {
                let len = Value::Int(self.s.len().try_into().unwrap());
                frame.op_stack.push(len);
            }
            ("toString", "()Ljava/lang/String;") => {
                // just return self
                popped_self = true; // lies but it works
            }
            _ => {
                // do funky stuff
                eprintln!("{}", format!("Use of unimplemented function: {name}{desc} in class String").red());
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
            Some(other) => self.s == other.s,
        }
    }

}

impl String {
    pub fn new_from_str(s: &str, jvm: &mut JVM ) -> Result<Rc<String>, Error> {
        Ok(Rc::new(String {s: std::string::String::from(s), 
        s_class: jvm.resolve_class_reference("java/lang/String")?}))
    }
    pub fn new_from_string(s: std::string::String, jvm: &mut JVM ) -> Result<Rc<String>, Error> {
        Ok(Rc::new(String {s, 
        s_class: jvm.resolve_class_reference("java/lang/String")?}))
    }
    pub fn backing_string(&self) -> &std::string::String {
        &self.s
    }
}