use std::time::{Instant, SystemTime, UNIX_EPOCH};

use super::*;
use crate::{errorcodes::Opcode, reference::{Reference, object::{self, natives}, Monitor, array::{Array, RefArray}}, access_macros};

pub struct System {
    pub instream: Reference<dyn Class, dyn Object>,
    pub outstream: Reference<dyn Class, dyn Object>,
    pub errstream: Reference<dyn Class, dyn Object>,
    console: Reference<dyn Class, dyn Object>,
    file: Rc<ClassFile>,
}

impl System {
    pub fn set_in(&mut self, obj_ref: Reference<dyn Class, dyn Object>) -> Result<(), Error> {
        if let Reference::Object(ref obj, _) = obj_ref {
            // Make sure the object class is preserved
            if !self.instream.is_null() && !Rc::ptr_eq(&self.instream.as_object().unwrap().class() , &obj.class()) {
                Err(Error::IncompatibleClassChangeError(Opcode::NativeMethod))
            } 
            else {
                self.instream = obj_ref;
                Ok(())
            }
        }
        else if obj_ref.is_null() {
            self.instream = Reference::Null;
            Ok(())
        }
        else {
            Err(Error::UnexpectedTypeOnStack(Opcode::NativeMethod))
        }
    }
    pub fn set_out(&mut self, obj_ref: Reference<dyn Class, dyn Object>) -> Result<(), Error> {
        if let Reference::Object(ref obj, _) = obj_ref {
            // Make sure the object class is preserved
            if !self.outstream.is_null() && !Rc::ptr_eq(&self.outstream.as_object().unwrap().class() , &obj.class()) {
                Err(Error::IncompatibleClassChangeError(Opcode::NativeMethod))
            } 
            else {
                self.outstream = obj_ref;
                Ok(())
            }
        }
        else if obj_ref.is_null() {
            self.outstream = Reference::Null;
            Ok(())
        }
        else {
            Err(Error::UnexpectedTypeOnStack(Opcode::NativeMethod))
        }
    }
    pub fn set_err(&mut self, obj_ref: Reference<dyn Class, dyn Object>) -> Result<(), Error> {
        if let Reference::Object(ref obj, _) = obj_ref {
            // Make sure the object class is preserved
            if !self.errstream.is_null() && !Rc::ptr_eq(&self.errstream.as_object().unwrap().class() , &obj.class()) {
                Err(Error::IncompatibleClassChangeError(Opcode::NativeMethod))
            } 
            else {
                self.errstream = obj_ref;
                Ok(())
            }
        }
        else if obj_ref.is_null() {
            self.errstream = Reference::Null;
            Ok(())
        }
        else {
            Err(Error::UnexpectedTypeOnStack(Opcode::NativeMethod))
        }
    }
    fn set_console(&mut self, obj_ref: Reference<dyn Class, dyn Object>) -> Result<(), Error> {
        if let Reference::Object(ref obj, _) = obj_ref {
            // Make sure the object class is preserved
            if !self.console.is_null() && !Rc::ptr_eq(&self.console.as_object().unwrap().class() , &obj.class()) {
                Err(Error::IncompatibleClassChangeError(Opcode::NativeMethod))
            } 
            else {
                self.console = obj_ref;
                Ok(())
            }
        }
        else if obj_ref.is_null() {
            self.console = Reference::Null;
            Ok(())
        }
        else {
            Err(Error::UnexpectedTypeOnStack(Opcode::NativeMethod))
        }
    }
    pub fn console(&self) -> Reference<dyn Class, dyn Object> {
        //FIXME: Instantiate console if null
        self.console.clone()
    }
    pub fn inherited_channel(&self) -> Reference<dyn Class, dyn Object> {Reference::Null}
    pub fn set_security_manager(&mut self, jvm: &mut JVM, obj_ref: Reference<dyn Class, dyn Object>) -> Result<(), Error> {
        // FIXME: Does this work?
        let e_obj = object::new_object("java/lang/UnsupportedOperationException", jvm)?;
        let e_ref = Reference::<dyn Class, dyn Object>::Object(e_obj, Rc::new(Monitor::new()));
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        frame.op_stack.push(Value::Reference(e_ref));
        Err(Error::Exception)
    } 
    pub fn get_security_manager(&self) -> Reference<dyn Class, dyn Object> {Reference::Null}
    pub fn current_time_millis(&self) -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }    
    pub fn nano_time(&self, jvm: &JVM) -> u128 {
        Instant::now().duration_since(jvm.start_time).as_nanos()
    }
    pub fn array_copy(&self, src_ref: Reference<dyn Class, dyn Object>, src_pos: i32, dest_ref: Reference<dyn Class, dyn Object>, 
        dest_pos: i32, len: i32, jvm: &mut JVM) -> Result<(), Error>
    {   
        let e_obj;
        if let Reference::Null = src_ref {
            e_obj = object::new_object("java/lang/NullPointerException", jvm)?;
        }
        else if let Reference::Null = dest_ref {
            e_obj = object::new_object("java/lang/NullPointerException", jvm)?;
        }
        else if let Reference::Array(mut src, _) = src_ref && let Reference::Array(mut dest, _) = dest_ref {
            if src_pos < 0 || dest_pos < 0 || len < 0 || 
            (src_pos + len) as usize > src.len() || (dest_pos + len) as usize > dest.len() {
                e_obj = object::new_object("java/lang/IndexOutOfBoundsException", jvm)?;
            }
            else {
                if Rc::ptr_eq(&src, &dest) {
                    let mut temp_dest: Array<dyn Class, dyn Object> = match &*src {
                        Array::Bool(_) => Array::Bool(Vec::with_capacity(src.len())),
                        Array::Byte(_) => Array::Byte(Vec::with_capacity(src.len())),
                        Array::Char(_) => Array::Char(Vec::with_capacity(src.len())),
                        Array::Double(_) => Array::Double(Vec::with_capacity(src.len())),
                        Array::Float(_) => Array::Float(Vec::with_capacity(src.len())),
                        Array::Int(_) => Array::Int(Vec::with_capacity(src.len())),
                        Array::Long(_) => Array::Long(Vec::with_capacity(src.len())),
                        Array::Ref(r) => Array::Ref(
                            RefArray { arr: Vec::with_capacity(src.len()), descriptor: r.descriptor.clone() }),
                        Array::Short(_) => Array::Short(Vec::with_capacity(src.len())),
                    };
                    for idx in 0..len {
                        temp_dest.set((dest_pos + idx) as usize, src.get((src_pos + idx) as usize))?;
                    }
                    src = Rc::new(temp_dest);
                }  
                for idx in 0..len {
                    let dest_mut = unsafe { Rc::get_mut_unchecked(&mut dest) };
                    dest_mut.set((dest_pos + idx) as usize, src.get((src_pos + idx) as usize))?;
                }
                return Ok(());
            }
        }
        else {
            e_obj = object::new_object("java/lang/ArrayStoreException", jvm)?;  
        }
        let e_ref = Reference::<dyn Class, dyn Object>::Object(e_obj, Rc::new(Monitor::new()));
        let thread = current_thread_mut!(jvm);
        let frame = current_frame_mut!(thread);
        frame.op_stack.push(Value::Reference(e_ref));
        Err(Error::Exception)
    }
}

impl Class for System {
    fn new(file: classfile::ClassFile, jvm: &mut JVM) -> Result<Self, Error> where Self : Sized {
        let out_object = natives::printstream::PrintStream::new_stdout(jvm)?;
        let out_ref = Reference::Object(out_object, Rc::new(Monitor::new()));
        Ok( System {
            instream: Reference::Null,
            outstream: out_ref,
            errstream: Reference::Null,
            console: Reference::Null,
            file: Rc::new(file),
        })
    }
    fn get_static(&self, name: &String, _descriptor: &String, _jvm: &mut JVM) -> Result<Value<dyn Class, dyn Object>, Error> {
        match name.as_str() {
            "in" => Ok(Value::Reference(self.instream.clone())),
            "out" => Ok(Value::Reference(self.outstream.clone())),
            "err" => Ok(Value::Reference(self.errstream.clone())),
            "console" => Ok(Value::Reference(self.console.clone())),
            _ => Err(Error::NoSuchFieldError(Opcode::NativeMethod)),
        }
    }
    fn put_static(&mut self, name: &String, _descriptor: &String, value:  Value<dyn Class, dyn Object>, _jvm: &mut JVM) -> Result<(), Error> {
        match name.as_str() {
            "in" => self.set_in(value.as_reference()?),
            "out" => self.set_out(value.as_reference()?),
            "err" => self.set_err(value.as_reference()?),
            "console" => self.set_console(value.as_reference()?),
            _ => Err(Error::NoSuchFieldError(Opcode::NativeMethod)),
        }
    }
    fn exec_method(self: Rc<Self>, jvm: &mut JVM, method: &MethodInfo) -> Result<bool, Error> {
        Err(Error::Todo(Opcode::NativeMethod))
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