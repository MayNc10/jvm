use std::rc::Rc;
use colored::Colorize;

use super::super::*;
use crate::{errorcodes::Opcode, reference::{Reference, object::{self, natives}, Monitor, array::{Array, RefArray}}, frame::Frame};

pub struct Double {
    file: Rc<ClassFile>
}

impl Class for Double {
    fn new(file: ClassFile, _jvm: &mut JVM) -> Result<Self, Error> where Self : Sized {
        Ok( Double {
            file: Rc::new(file),
        })
    }
    fn get_static(&self, name: &str, _descriptor: &str, _jvm: &mut JVM) -> Result<Value<dyn Class, dyn Object>, Error> {
        match name {
            _ => Err(Error::NoSuchFieldError(Opcode::NativeMethod)),
        }
    }
    fn put_static(&mut self, name: &str, _descriptor: &str, _value:  Value<dyn Class, dyn Object>, _jvm: &mut JVM) -> Result<(), Error> {
        match name {
            _ => Err(Error::NoSuchFieldError(Opcode::NativeMethod)),
        }
    }
    fn exec_method(self: Rc<Self>, jvm: &mut JVM, method: &MethodInfo) -> Result<bool, Error> {
        let name = self.file.cp_entry(method.name_index)?.as_utf8()?.as_str();
        let desc = self.file.cp_entry(method.descriptor_index)?.as_utf8()?.as_str();
        let thread = current_thread_mut!(jvm);
        let frame: &mut Frame = current_frame_mut!(thread);
        let mut was_natively_executed = true;
        match (name, desc) {
            ("doubleToRawLongBits", "(D)J") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Long(val.to_bits().try_into().unwrap()));
            }
            _ => {
                // do funky stuff
                eprintln!("{}", format!("Use of unimplemented function: {name}{desc} in class Double").red());
                if &desc[desc.len() - 1..] != "V" {
                    // expected to push something onto stack
                    frame.op_stack.push(Value::Reference(Reference::Null));
                }
                was_natively_executed = false;
            }
        }
        Ok(was_natively_executed)
    }
    fn get_class_file(&self) -> Rc<ClassFile> {
        Rc::clone(&self.file)
    }
    fn as_any(&self) ->  &dyn Any {
        self
    }
    fn as_any_rc(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn Class> {
        self
    }
}
