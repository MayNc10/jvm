use crate::reference::object::natives;
use crate::reference::object::Object;
use super::*;

use colored::Colorize;

#[derive(Debug, PartialEq, Clone)]
pub struct Nop {}
impl Instruction for Nop {
    fn name(&self) -> &'static str {
        "nop"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if !was_wide {
            Ok(Nop {})
        } else {
            Err(Error::IllegalWide)
        }
    }
    fn execute(&mut self, _jvm : &mut super::JVM) -> Result<(), crate::errorcodes::Error> {
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Nop>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct AConstNull {}
impl Instruction for AConstNull {
    fn name(&self) -> &'static str {
        "aconst_null"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<AConstNull>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct IConstM1 {}
impl Instruction for IConstM1 {
    fn name(&self) -> &'static str {
        "iconst_m1"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<IConstM1>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct IConst0 {}
impl Instruction for IConst0 {
    fn name(&self) -> &'static str {
        "iconst_0"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<IConst0>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct IConst1 {}
impl Instruction for IConst1 {
    fn name(&self) -> &'static str {
        "iconst_1"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<IConst1>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct IConst2 {}
impl Instruction for IConst2 {
    fn name(&self) -> &'static str {
        "iconst_2"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<IConst2>() {
            None => false,
            Some(other) => self == other,
        }
    }
    fn can_jit(&self) -> bool { true }
    #[cfg(not(target_family = "wasm"))]
    fn jit(&self, context: &'static Context, module: &Module<'static>, builder: &Builder<'static>, 
                engine: &ExecutionEngine<'static>, name: &String, func: FunctionValue, 
                locals: &Vec<PointerValue>, blocks: &HashMap<usize, BasicBlock>, stack: &PointerValue, top: &PointerValue) 
    {
        builder.build_store(unsafe {
            builder.build_in_bounds_gep(*stack, &[
                builder.build_load(*top, "top_idx").into_int_value()
            ], "stack_idx")
        }, context.i32_type().const_int(2, false));

        builder.build_store(*top, 
            builder.build_load(*top, "top_idx").into_int_value().const_add(
                context.i64_type().const_int(1, false)
            ), 
        );
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct IConst3 {}
impl Instruction for IConst3 {
    fn name(&self) -> &'static str {
        "iconst_3"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<IConst3>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct IConst4 {}
impl Instruction for IConst4 {
    fn name(&self) -> &'static str {
        "iconst_4"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<IConst4>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct IConst5 {}
impl Instruction for IConst5 {
    fn name(&self) -> &'static str {
        "iconst_5"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<IConst5>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct LConst0 {}
impl Instruction for LConst0 {
    fn name(&self) -> &'static str {
        "lconst_0"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<LConst0>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct LConst1 {}
impl Instruction for LConst1 {
    fn name(&self) -> &'static str {
        "lconst_1"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<LConst1>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct FConst0 {}
impl Instruction for FConst0 {
    fn name(&self) -> &'static str {
        "fconst_0"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<FConst0>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct FConst1 {}
impl Instruction for FConst1 {
    fn name(&self) -> &'static str {
        "fconst_1"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<FConst1>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct FConst2 {}
impl Instruction for FConst2 {
    fn name(&self) -> &'static str {
        "fconst_2"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<FConst2>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct DConst0 {}
impl Instruction for DConst0 {
    fn name(&self) -> &'static str {
        "dconst_0"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<DConst0>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct DConst1 {}
impl Instruction for DConst1 {
    fn name(&self) -> &'static str {
        "dconst_1"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<DConst1>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct BiPush {byte: i32}
impl Instruction for BiPush {
    fn name(&self) -> &'static str {
        "bipush"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<BiPush>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct SiPush {short: i32}
impl Instruction for SiPush {
    fn name(&self) -> &'static str {
        "sipush"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<SiPush>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

pub mod ldc {
    use std::fmt::{Debug, Display};

    use super::*;

    macro_rules! comparable {
        () => {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn eq(&self, other: &dyn LDCFunc) -> bool {
                match other.as_any().downcast_ref::<Self>() {
                    None => false,
                    Some(other) => self == other,
                }
            }   
        }
    }

    pub trait LDCFunc : Debug + DynClone {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error>;
        fn as_any(&self) -> &dyn Any;
        fn eq(&self, other: &dyn LDCFunc) -> bool;
    }

    impl PartialEq for dyn LDCFunc {
        fn eq(&self, other: &Self) -> bool {
            self.eq(other)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
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
        comparable!{}
    }
    #[derive(Debug, PartialEq, Clone)]
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
        comparable!{}
    }
    #[derive(Clone, PartialEq)]
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
        comparable!{}
    }
    impl Display for LDCString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LDCString \"{}\"", unsafe {
            self.s.as_object().unwrap().as_any().downcast_ref_unchecked::<natives::string::String>().backing_string()})
        }
    }
    impl Debug for LDCString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self}")
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct LDCClass {
        pub c_name: String,
    }
    impl LDCFunc for LDCClass {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
            jvm.gen_class_obj(self.c_name.as_str())
        }
        comparable!{}
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct LDCMethodType {
        pub desc: &'static str, // Same idea as above
    }
    impl LDCFunc for LDCMethodType {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
            let desc = String::from(self.desc);
            let (params_s, ret_s) = desc.split_at(desc.rfind(')').unwrap());
            let params_s = String::from(params_s);
            // In makeimpl, it takes a bool asking whether the params are trusted. I think they are in this case.
            jvm.gen_class_obj(ret_s)?;
            let idx = 0;
            // This logic should be merged with setup_method_call
            while idx < params_s.len() {

            }
            Ok(())
        }
        comparable!{}
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct LDCMethodHandle;
    #[derive(Debug, PartialEq, Clone)]
    pub struct LDCDynamic;
    #[derive(Debug, PartialEq, Clone)]
    pub struct LDCDouble {
        pub d: f64,
    }
    impl LDCFunc for LDCDouble {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
            let thread = access_macros::current_thread_mut!(jvm);
            let frame = access_macros::current_frame_mut!(thread);
            frame.op_stack.push(Value::Double(self.d));
            Ok(())
        }
        comparable!{}
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct LDCLong {
        pub l: i64,
    }
    impl LDCFunc for LDCLong {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
            let thread = access_macros::current_thread_mut!(jvm);
            let frame = access_macros::current_frame_mut!(thread);
            frame.op_stack.push(Value::Long(self.l));
            Ok(())
        }
        comparable!{}
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct LDCTodo {}
    impl LDCFunc for LDCTodo {
        fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
            // Just to try to get farther
            eprintln!("{}", "WARNING: LDCTodo called, attempting to fudge results".red());

            let thread = access_macros::current_thread_mut!(jvm);
            let frame = access_macros::current_frame_mut!(thread);
            frame.op_stack.push(Value::Reference(Reference::Null)); // The only unimplemented ldcs are reference ones.
            Ok(())
        }
        comparable!{}
    }
    clone_trait_object!(LDCFunc);
}

#[derive(Debug, Clone)]
pub struct Ldc {
    f: Box<dyn ldc::LDCFunc> // Has to be a ptr for size reasons. This still should be performant. 
}
impl Instruction for Ldc {
    fn name(&self) -> &'static str {
        "ldc"
    }
    fn new(v: &mut Vec<u8>, cpool: &Vec<Entry>, jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        }
        else {
            // First, get the constant pool entry at that index.
            let entry = &cpool[v[0] as usize - 1];
            let f = match entry {
                Entry::Integer(i) => Box::new(ldc::LDCInt {i: *i}) as Box<dyn ldc::LDCFunc>,
                Entry::Float(f) => Box::new(ldc::LDCFloat {f: *f}) as Box<dyn ldc::LDCFunc>,
                Entry::String(s) => Box::new(ldc::LDCString {
                        s: Reference::Object(
                           natives::string::String::new_from_string(cpool[*s as usize - 1].as_utf8()?.clone(), jvm)?, 
                        Rc::new(Monitor::new())) 
                }),
                Entry::Class(_c) => {
                    // The spec says we have to return a reference to the class or interface itself. 
                    // I think it means that we have to create a java.lang.Class object and return a reference.
                    // For the same reasons as string above, we are not implementing this right now.
                    Box::new(ldc::LDCTodo {}) as Box<dyn ldc::LDCFunc>
                },
                // For these next 2, see https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.5
                Entry::MethodType(_m) => {
                    // This one is a reference to java.lang.invoke.MethodType.
                    // See above.
                    Box::new(ldc::LDCTodo {}) as Box<dyn ldc::LDCFunc>
                },
                Entry::MethodHandle(_m) => {
                    // This one is incredibly complicated, but should result in a java.lang.invoke.MethodHandle.
                    Box::new(ldc::LDCTodo {}) as Box<dyn ldc::LDCFunc>
                },
                Entry::MethodRef(_m) => {
                    // Apparently this is loadable
                    Box::new(ldc::LDCTodo {}) as Box<dyn ldc::LDCFunc>
                }
                Entry::Dynamic(_dynamic) => {
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
                    Box::new(ldc::LDCTodo {}) as Box<dyn ldc::LDCFunc>
                },
                Entry::Long(_) | Entry::Double(_) => {
                    // Even though these are loadable, they shouldn't appear here
                    return Err(Error::IllegalConstantLoad(Opcode::LDC));
                },
                _ => {
                    return Err(Error::IllegalConstantLoad(Opcode::LDC));
                },
            };
            v.remove(0);
            Ok(Ldc { f })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        self.f.execute(jvm)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Ldc>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
impl PartialEq for Ldc {
    fn eq(&self, other: &Self) -> bool {
        &*self.f == &*other.f
    }
}
#[derive(Debug, Clone)]
pub struct LdcW {
    f: Box<dyn ldc::LDCFunc> // Has to be a ptr for size reasons. This still should be performant. 
}
impl Instruction for LdcW {
    fn name(&self) -> &'static str {
        "ldc_w"
    }
    fn new(v: &mut Vec<u8>, cpool: &Vec<Entry>, jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        }
        else {
            // First, get the constant pool entry at that index.
            let entry = &cpool[unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap())
            } as usize - 1];
            
            let f = match entry {
                Entry::Integer(i) => Box::new(ldc::LDCInt {i: *i}) as Box<dyn ldc::LDCFunc>,
                Entry::Float(f) => Box::new(ldc::LDCFloat {f: *f}) as Box<dyn ldc::LDCFunc>,
                Entry::String(s) => Box::new(ldc::LDCString {
                        s: Reference::Object(
                           natives::string::String::new_from_string(cpool[*s as usize - 1].as_utf8()?.clone(), jvm)?, 
                        Rc::new(Monitor::new())) 
                }),
                Entry::Class(_c) => {
                    // The spec says we have to return a reference to the class or interface itself. 
                    // I think it means that we have to create a java.lang.Class object and return a reference.
                    // For the same reasons as string above, we are not implementing this right now.
                    Box::new(ldc::LDCTodo {}) as Box<dyn ldc::LDCFunc>
                },
                // For these next 2, see https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.5
                Entry::MethodType(_m) => {
                    // This one is a reference to java.lang.invoke.MethodType.
                    // See above.
                    Box::new(ldc::LDCTodo {}) as Box<dyn ldc::LDCFunc>
                },
                Entry::MethodHandle(_m) => {
                    // This one is incredibly complicated, but should result in a java.lang.invoke.MethodHandle.
                    Box::new(ldc::LDCTodo {}) as Box<dyn ldc::LDCFunc>
                },
                /* 
                Entry::MethodRef(_r) => {
                    // Apparently this is allowed?
                    Box::new(ldc::LDCTodo {}) as Box<dyn ldc::LDCFunc>
                }
                */
                Entry::Dynamic(_dynamic) => {
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
                    Box::new(ldc::LDCTodo {}) as Box<dyn ldc::LDCFunc>
                },
                Entry::Long(_) | Entry::Double(_) => {
                    // Even though these are loadable, they shouldn't appear here
                    return Err(Error::IllegalConstantLoad(Opcode::LDCW));
                },
                _ => {
                    return Err(Error::IllegalConstantLoad(Opcode::LDCW));
                },
            };
            v.remove(0);
            v.remove(0);
            Ok(LdcW { f })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        self.f.execute(jvm)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<LdcW>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
impl PartialEq for LdcW {
    fn eq(&self, other: &Self) -> bool {
        &*self.f == &*other.f
    }
}
#[derive(Debug, Clone)]
pub struct Ldc2W {
    f: Box<dyn ldc::LDCFunc> // Has to be a ptr for size reasons. This still should be performant. 
}
impl Instruction for Ldc2W {
    fn name(&self) -> &'static str {
        "ldc2_w"
    }
    fn new(v: &mut Vec<u8>, cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        }
        else {
            // First, get the constant pool entry at that index.
            let entry = &cpool[unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) as usize
            } - 1];
            let f = match entry {
                Entry::Double(d) => Box::new(ldc::LDCDouble {d: *d}) as Box<dyn ldc::LDCFunc>,
                Entry::Long(l) => Box::new(ldc::LDCLong {l: *l}) as Box<dyn ldc::LDCFunc>,
                Entry::Dynamic(_dynamic) => {
                    // like LdcW dynamic, except it can only load references to longs or doubles
                    Box::new(ldc::LDCTodo {}) as Box<dyn ldc::LDCFunc>
                },
                Entry::Integer(_) | Entry::Float(_) | Entry::String(_) | Entry::Class(_) | 
                Entry::MethodHandle(_) | Entry::MethodType(_) => return Err(Error::IllegalConstantLoad(Opcode::LDC2W)),
                _ => return Err(Error::IllegalConstantLoad(Opcode::LDC2W)),
            };
            v.remove(0);
            v.remove(0);
            Ok(Ldc2W { f })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        self.f.execute(jvm)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Ldc2W>() {
            None => false,
            Some(other) => self == other,
        }
    }
}
impl PartialEq for Ldc2W {
    fn eq(&self, other: &Self) -> bool {
        &*self.f == &*other.f
    }
}


