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
        frame.op_stack.push(Value::Reference(Rc::new(Reference::Null)));
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
        s: &'static str // This should hopefully work, if not idk. 
    }
    impl LDCFunc for LDCString {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
            let s_obj = Object::new_with_name("java/lang/String", jvm)?;
            let string_class = Rc::clone(&s_obj.m_class);
            let s_ref = Reference::Object(s_obj.clone(), Rc::new(Monitor::new()));             
            let carray = Array::Char(self.s.encode_utf16().collect());
            let carray_len = carray.len();
            let carray_asref = Reference::Array(carray, Rc::new(Monitor::new()));
            let mut s_ref_rc = Rc::new(s_ref);
            let thread = access_macros::current_thread_mut!(jvm);
            let frame = access_macros::current_frame_mut!(thread);
            frame.op_stack.push(Value::Reference(Rc::clone(&mut s_ref_rc)));
            frame.op_stack.push(Value::Reference(s_ref_rc)); // duplicate the value, so we still have one afterwards.
            frame.op_stack.push(Value::Reference(Rc::new(carray_asref)));
            frame.op_stack.push(Value::Int(0));
            frame.op_stack.push(Value::Int(carray_len as i32));
            jvm.setup_method_call_from_name("<init>", "([CII)V", Rc::clone(&string_class) , false)?;
            jvm.run_until_method_exit();
            // Lastly, we have to call .intern() on it.
            // String <init> calls this already, so I don't think we have to call it.
            // jv,.execute_native_from_name("intern", "()Ljava/lang/String;", string_class)?;
            Ok(())
        }
    }
    pub struct LDCClass {
        c_name: &'static str, // same idea as LDCString
    }
    impl LDCFunc for LDCClass {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
            jvm.gen_class_obj(self.c_name)
        }
    }
    pub struct LDCMethodType {
        desc: &'static str, // Same idea as above
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

pub struct LDC<F: ldc::LDCFunc> {
    f: Box<F> // Has to be a ptr for size reasons. This still should be performant. 
}
impl<F: ldc::LDCFunc> Instruction for LDC<F> {
    fn name(&self) -> &'static str {
        "ldc"
    }
    fn new(v: &mut Vec<u8>, _c: Rc<dyn Class>, _jvm: &mut JVM, was_wide: bool) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        }
        else {

        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        self.f.execute(jvm)
    }
}
