use crate::reference::object::natives;

use super::*;

pub struct Nop {}
impl Instruction for Nop {
    fn name(&self) -> &'static str {
        "nop"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(Nop {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, _jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        Ok(())
    }
}

pub struct AConstNull {}
impl Instruction for AConstNull {
    fn name(&self) -> &'static str {
        "aconst_null"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(AConstNull {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);  
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Reference(Reference::Null));
        Ok(())
    }
}

pub struct IConstM1 {}
impl Instruction for IConstM1 {
    fn name(&self) -> &'static str {
        "iconst_m1"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(IConstM1 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);       
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Int(-1));
        Ok(())
    }
}
pub struct IConst0 {}
impl Instruction for IConst0 {
    fn name(&self) -> &'static str {
        "iconst_0"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(IConst0 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Int(0));
        Ok(())
    }
}

pub struct IConst1 {}
impl Instruction for IConst1 {
    fn name(&self) -> &'static str {
        "iconst_1"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(IConst1 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Int(1));
        Ok(())
    }
}

pub struct IConst2 {}
impl Instruction for IConst2 {
    fn name(&self) -> &'static str {
        "iconst_2"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(IConst2 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Int(2));
        Ok(())
    }
}

pub struct IConst3 {}
impl Instruction for IConst3 {
    fn name(&self) -> &'static str {
        "iconst_3"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(IConst3 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Int(3));
        Ok(())
    }
}

pub struct IConst4 {}
impl Instruction for IConst4 {
    fn name(&self) -> &'static str {
        "iconst_4"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(IConst4 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Int(4));
        Ok(())
    }
}

pub struct IConst5 {}
impl Instruction for IConst5 {
    fn name(&self) -> &'static str {
        "iconst_5"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(IConst5 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Int(5));
        Ok(())
    }
}

pub struct LConst0 {}
impl Instruction for LConst0 {
    fn name(&self) -> &'static str {
        "lconst_0"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(LConst0 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Long(0));
        Ok(())
    }
}

pub struct LConst1 {}
impl Instruction for LConst1 {
    fn name(&self) -> &'static str {
        "lconst_1"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(LConst1 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Long(1));
        Ok(())
    }
}

pub struct FConst0 {}
impl Instruction for FConst0 {
    fn name(&self) -> &'static str {
        "fconst_0"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(FConst0 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Float(0.0));
        Ok(())
    }
}

pub struct FConst1 {}
impl Instruction for FConst1 {
    fn name(&self) -> &'static str {
        "fconst_1"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(FConst1 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Float(1.0));
        Ok(())
    }
}

pub struct FConst2 {}
impl Instruction for FConst2 {
    fn name(&self) -> &'static str {
        "fconst_2"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(FConst2 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Float(2.0));
        Ok(())
    }
}

pub struct DConst0 {}
impl Instruction for DConst0 {
    fn name(&self) -> &'static str {
        "dconst_0"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(DConst0 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Double(0.0));
        Ok(())
    }
}

pub struct DConst1 {}
impl Instruction for DConst1 {
    fn name(&self) -> &'static str {
        "dconst_1"
    }
    fn new(_v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(DConst1 {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Double(1.0));
        Ok(())
    }
}

pub struct BiPush {byte: i32}
impl Instruction for BiPush {
    fn name(&self) -> &'static str {
        "bipush"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        let b = v.remove(0);      
        if !was_wide {
            Ok(BiPush { byte: b as i32})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Byte(self.byte));
        Ok(())
    }
}

pub struct SiPush {short: i32}
impl Instruction for SiPush {
    fn name(&self) -> &'static str {
        "sipush"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        let s = unsafe {
            i16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())  
        };
        v.remove(0);
        v.remove(0);
        if !was_wide {
            Ok(SiPush { short: s as i32})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Short(self.short));
        Ok(())
    }
}

pub mod ldc {
    use super::*;
    pub trait LDCFunc {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error>;
    }
    pub struct LDCInt {
        pub i: i32
    } 
    impl LDCFunc for LDCInt {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
            let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Int(self.i));
        Ok(())
        }
    }
    pub struct LDCFloat {
        pub f: f32
    } 
    impl LDCFunc for LDCFloat {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
            let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Float(self.f));
        Ok(())
        }
    }
    pub struct LDCString {
        pub s: Reference<dyn Class, dyn Object>, 
    }
    impl LDCFunc for LDCString {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
            let thread = current_thread_mut!(jvm);
            let frame = current_frame_mut!(thread);
            frame.op_stack.push(Value::Reference(self.s.clone()));
            Ok(())
        }
    }
    pub struct LDCClass {
        pub c_name: String,
    }
    impl LDCFunc for LDCClass {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
            jvm.gen_class_obj(self.c_name.as_str())
        }
    }
    pub struct LDCMethodType {
        pub desc: &'static str, // Same idea as above
    }
    impl LDCFunc for LDCMethodType {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
            let desc = String::from(self.desc);
            let (params_s, ret_s) = desc.split_at(desc.rfind(")").unwrap());
            let params_s = String::from(params_s);
            // In makeimpl, it takes a bool asking whether the params are trusted. I think they are in this case.
            jvm.gen_class_obj(ret_s);
            let idx = 0;
            // This logic should be merged with setup_method_call
            while idx < params_s.len() {

            }
            Ok(())
        }
    }
    pub struct LDCMethodHandle;
    pub struct LDCDynamic;

}

pub struct LDC<F: ldc::LDCFunc + ?Sized> {
    f: Box<F> // Has to be a ptr for size reasons. This still should be performant. 
}
impl Instruction for LDC<dyn ldc::LDCFunc> {
    fn name(&self) -> &'static str {
        "ldc"
    }
    fn new(v: &mut Vec<u8>, c: Rc<dyn Class>, jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        }
        else {
            // First, get the constant pool entry at that index.
            let c_file = c.get_class_file();
            let entry = c_file.cp_entry(unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())  
            })?;
            let f = match entry {
                Entry::Integer(i) => Box::new(ldc::LDCInt {i: *i}) as Box<dyn ldc::LDCFunc>,
                Entry::Float(f) => Box::new(ldc::LDCFloat {f: *f}) as Box<dyn ldc::LDCFunc>,
                Entry::String(s) => Box::new(ldc::LDCString {
                        s: Reference::Object(
                           natives::string::String::new_from_string(c.get_class_file().cp_entry(*s)?.as_utf8()?.clone(), jvm)?, 
                        Rc::new(Monitor::new())) 
                }),
                Entry::Class(c) => {
                    // The spec says we have to return a reference to the class or interface itself. 
                    // I think it means that we have to create a java.lang.Class object and return a reference.
                    // For the same reasons as string above, we are not implementing this right now.
                    return Err(Error::Todo(Opcode::LDC));
                },
                // For these next 2, see https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.5
                Entry::MethodType(m) => {
                    // This one is a reference to java.lang.invoke.MethodType.
                    // See above.
                    return Err(Error::Todo(Opcode::LDC));
                },
                Entry::MethodHandle(m) => {
                    // This one is incredibly complicated, but should result in a java.lang.invoke.MethodHandle.
                    return Err(Error::Todo(Opcode::LDC));
                },
                Entry::Dynamic(dynamic) => {
                    // This Dynamic cannot reference a field with discriptor Long or Double.
    
                    // For more information about the process see: https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.6
                    // First, we have to find a 'bootstrap method' to call to produce the value.
                    // This is done by indexing into the BootstrapMethods Attribute of the current class.
                    // This gives us a methodhandle info and a list of static arguments (loadable entries in the constant pool).
                    // The method handle is resolved in the same way as above, except that the first parameter of the method must be java.lang.invoke.MethodHandles.Lookup.
                    // If it's not, we fail with a BootstrapMethodError.
                    // We alse get a field descriptor, from which we create a java.lang.Class object from it.
                    // We then resolve every static argument. This process can be recursive, so we should make it a function.
                    // Second, we have to call the bootstrap method.
                    // To do this, we first create an Array of Object with length n + 3, where n is the number of static arguments.
                    // The 0[] index is a reference an instance of java.lang.invoke.MethodHandles.Lookup for the current class.
                    // The 1[] index is an reference to an instance of java.lang.String denoting the unqualified name from the name and type info.
                    // The 2[] index is the reference to and instance of Class obtained earlier.
                    // The rest of the array is filled with the static arguments.
                    // The method handle is invoked with BMH.invokeWithArguments(args).
                    // Finally, we have to validate the reference produced by the invocation.
                    // the reference o is converted as if by invoking MH.invoke(o), 
                    // where MH is a method handler produced from invoking the identity(class Object) method of java.lang.invoke.MethodHandles.
                    // If this gives a NullPtrException or a ClassCastExeption, we fail with a BootstrapMethodError.
                    return Err(Error::Todo(Opcode::LDC));
                },
                Entry::Long(_) | Entry::Double(_) => {
                    // Even though these are loadable, they shouldn't appear here
                    return Err(Error::IllegalConstantLoad(Opcode::LDCW));
                },
                _ => {
                    return Err(Error::IllegalConstantLoad(Opcode::LDCW));
                },
            };
            Ok(LDC { f })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        self.f.execute(jvm)
    }
}
