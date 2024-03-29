use crate::{errorcodes::Opcode, access_macros, frame::Frame};

use super::super::*;

#[derive(PartialEq)]
enum PrintStreamInner {
    Stdout,
    Stderr,
    File(Reference<dyn Class, dyn Object>),
}

impl PrintStreamInner {
    pub fn println(&self, s: &natives::string::String) -> Result<(), Error> {
        match self {
            PrintStreamInner::Stdout => println!("{}", s.backing_string()),
            PrintStreamInner::Stderr => eprintln!("{}", s.backing_string()),
            PrintStreamInner::File(_f) => return Err(Error::Todo(Opcode::NativeMethod)),
        }
        Ok(())
    } 
    pub fn print(&self, s: &natives::string::String) -> Result<(), Error> {
        match self {
            PrintStreamInner::Stdout => print!("{}", s.backing_string()),
            PrintStreamInner::Stderr => eprint!("{}", s.backing_string()),
            PrintStreamInner::File(_f) => return Err(Error::Todo(Opcode::NativeMethod)),
        }
        Ok(())
    } 
    pub fn print_double(&self, d: &f64) -> Result<(), Error> {
        //println!("Printing double with value {}", d);
        let d = *d;
        if d == d.round() {
            match self {
                PrintStreamInner::Stdout => print!("{d:.1}"),
                PrintStreamInner::Stderr => eprint!("{d:.1}"),
                PrintStreamInner::File(_f) => return Err(Error::Todo(Opcode::NativeMethod)),
            }
        }
        else {
            match self {
                PrintStreamInner::Stdout => print!("{d}"),
                PrintStreamInner::Stderr => eprint!("{d}"),
                PrintStreamInner::File(_f) => return Err(Error::Todo(Opcode::NativeMethod)),
            }
        }
        
        Ok(())
    } 
    pub fn print_int(&self, i: &i32) -> Result<(), Error> {
        match self {
            PrintStreamInner::Stdout => print!("{i}"),
            PrintStreamInner::Stderr => eprint!("{i}"),
            PrintStreamInner::File(_f) => return Err(Error::Todo(Opcode::NativeMethod)),
        }
        Ok(())
    } 
    pub fn newline(&self) -> Result<(), Error> {
        match self {
            PrintStreamInner::Stdout => println!(),
            PrintStreamInner::Stderr => eprintln!(),
            PrintStreamInner::File(_f) => return Err(Error::Todo(Opcode::NativeMethod)),
        }
        Ok(())
    }
}

pub struct PrintStream {
    inner: PrintStreamInner,
    class: Rc<dyn Class>,
}

impl Object for PrintStream {
    fn new(_current_method_class: Option<Rc<dyn Class>>, _class_index: Option<u16>, jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self : Sized {
        Ok(Rc::new(PrintStream {
            inner: PrintStreamInner::File(Reference::Null),
            class: jvm.resolve_class_reference("java/io/PrintStream")?,
        }))
    }
    fn new_with_name(_name: &str, _jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self : Sized {
        panic!("No one should ever call PrintStream::new_with_name")
    }
    fn get_field(&self, _current_method_class: Rc<dyn Class>, _class_index: u16, _jvm: &mut JVM) -> Result<Value<dyn Class, dyn Object>, Error> {
        Err(Error::Todo(Opcode::NativeMethod))
    }
    fn put_field(&mut self, _current_method_class: Rc<dyn Class>, _class_index: u16, _jvm: &mut JVM, _value: Value<dyn Class, dyn Object>) -> Result<(), Error> {
        Err(Error::Todo(Opcode::NativeMethod))
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
            ("print", "(Ljava/lang/String;)V") => {
                self.inner.print(frame.op_stack.pop().unwrap().as_reference()?.as_object().unwrap().as_any().downcast_ref::<natives::string::String>().unwrap())?;
            },
            ("print", "(D)V") => {
                self.inner.print_double(frame.op_stack.pop().unwrap().as_double()?)?;
            },  
            ("print", "(I)V") => {
                self.inner.print_int(frame.op_stack.pop().unwrap().as_int()?)?;
            },
            ("println", "(Ljava/lang/String;)V") => {
                self.inner.println(frame.op_stack.pop().unwrap().as_reference()?.as_object().unwrap().as_any().downcast_ref::<natives::string::String>().unwrap())?;
            },
            ("println", "(I)V") => {
                self.inner.print_int(frame.op_stack.pop().unwrap().as_int()?)?;
                self.inner.newline()?;
            },
            ("println", "(D)V") => {
                self.inner.print_double(frame.op_stack.pop().unwrap().as_double()?)?;
                self.inner.newline()?;
            },
            ("println", "()V") => {
                self.inner.newline()?;
            },
            
            
            _ => {
                // do funky stuff
                eprintln!("{}", format!("Use of unimplemented function: {name}{desc} in class PrintStream").red());
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
        Rc::clone(&self.class)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn into_any_rc(self: Rc<Self>) -> Rc<dyn Object> {
        self
    }
    fn is_equal(&self, other: &dyn Object) -> bool {
        match other.as_any().downcast_ref::<PrintStream>() {
            Some(other) => self.inner == other.inner,
            None => false
        }
    }
}

impl PrintStream {
    pub fn new_stdout(jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self : Sized {
        Ok(Rc::new(PrintStream {
            inner: PrintStreamInner::Stdout,
            class: jvm.resolve_class_reference("java/io/PrintStream")?,
        }))
    }
    pub fn new_stderr(jvm: &mut JVM) -> Result<Rc<dyn Object>, Error> where Self : Sized {
        Ok(Rc::new(PrintStream {
            inner: PrintStreamInner::Stderr,
            class: jvm.resolve_class_reference("java/io/PrintStream")?,
        }))
    }
    pub fn new_with_file(jvm: &mut JVM, file: Reference<dyn Class, dyn Object>) -> Result<Rc<dyn Object>, Error> where Self : Sized {
        Ok(Rc::new(PrintStream {
            inner: PrintStreamInner::File(file),
            class: jvm.resolve_class_reference("java/io/PrintStream")?,
        }))
    }
}