use crate::reference::array::Array;
use crate::{access_macros, class};
use crate::attributes::code::Exception;
use crate::errorcodes::{Error, Opcode};
use crate::class::{Class, MethodInfo};
use crate::constant_pool::Entry;
use crate::frame::Frame;
use crate::reference::{Reference, Monitor, array};
use crate::reference::object::Object;
use crate::thread::Thread;
use crate::value::{Value, VarValue};

use std::collections::HashMap;
use std::fs::{File, self};
use std::io::Read;
use std::option::Option; 
use std::rc::Rc;
use std::result::Result;
use std::string::String;
use std::vec::Vec;

// Just useful for code readability
mod operations;
pub mod settings;

const STEP_SIZE: usize = 10;

pub struct Crash {
    has_crashed: bool,
    crash_reason: String,
    base_traceback: String,
} 

pub struct JVM {
    m_threads: Vec<Thread>,
    pub m_loaded_classes: HashMap<String, Rc< Class>>,
    m_thrown_error: Error,
    m_crash_info: Crash,
    m_thread_index: usize,
    m_step_size: usize,
    m_has_halted: bool,
    m_main_class_name: String,
    m_flags: u8,
}

// TODO: Rework every dump function to use traits?

impl<'a> JVM {
    pub fn new_jvm(n: String, flags: u8) -> JVM {
        JVM {
            m_threads: vec![Thread::new()
            ],
            m_loaded_classes: HashMap::new(),
            m_thrown_error: Error::None,
            m_crash_info: Crash{ has_crashed: false, 
                crash_reason: String::from(""), 
                base_traceback: String::from("")},
            m_thread_index: 0,
            m_step_size: STEP_SIZE,
            m_has_halted: false,
            m_main_class_name: n,
            m_flags: flags,
        }
    }
    pub fn new_with_step_size(n: String, step_size: usize, flags: u8) -> JVM {
        JVM {
            m_threads: Vec::new(),
            m_loaded_classes: HashMap::new(),
            m_thrown_error: Error::None,
            m_crash_info: Crash{ has_crashed: false, 
                crash_reason: String::from(""), 
                base_traceback: String::from("")},
            m_thread_index: 0,
            m_step_size: step_size,
            m_has_halted: false,
            m_main_class_name: n,
            m_flags: flags,
        }
    }
    pub fn new_with_main_class(c: Class, flags: u8) -> Result<JVM, Error> {
        let mut jvm = Self::new_jvm(String::from(c.name()), flags);
        let name = String::from(c.name());
        jvm.m_loaded_classes.insert(name, Rc::new(c));
        // After adding the class, we initialize the class
        // We should use this error instead of ignoring it.
        let mut found_clinit = true;
        if let Err(e) = jvm.setup_method_call_from_name_on_main("<clinit>", "()V", true) {
            if e != Error::NoSuchMethodError(Opcode::MethodInvoke) {
                println!("{:#?}", e);
                return Err(e);
            }
            found_clinit = false;
        }
        if found_clinit {
            jvm.run_until_method_exit();
            if jvm.has_encoutered_error() {
                println!("Encountered error: {:#?}", jvm.m_thrown_error);
            }
        }
        Ok(jvm)
    }
    pub fn load_class_file(this_loaded_classes: &mut HashMap<String, Rc<Class>>, path: &str) -> Result<(), Error> {
        let mut resolved_path = String::from(path);
        if &path[0..4] == "java" {
            //TODO: Add java class path
            // For now we just hardcode the directory, in the future we will have to change that
            let mut temp_str = String::from("../classlibs/17/java.base/");
            temp_str.push_str(&resolved_path);
            resolved_path = temp_str;
        }
        drop(path); // This is just to make sure we don't use it again, as it isn't correct.
        resolved_path.push_str(".class");
        let mut f = match File::open(resolved_path.as_str()) {
            Ok(file) => file,
            Err(_) => {
                println!("Path: {}", path);
                return Err(Error::NoClassDefFoundError(Opcode::ClassLoad))
            },
        };
        let metadata = fs::metadata(&resolved_path).unwrap();
        let size = if metadata.len() % 8 == 0 {metadata.len() / 8} else {metadata.len() / 8 + 1};
        let buffer: Vec<u64> = vec![0; size as usize];
        let c = unsafe {
            let mut buf_bytes = std::slice::from_raw_parts_mut(buffer.as_ptr() as *mut u8, buffer.len() * std::mem::size_of::<u64>());
            f.read(&mut buf_bytes).unwrap();
            Class::new(&buf_bytes).unwrap()
        };
        this_loaded_classes.insert(String::from(c.name()), Rc::new(c));
        Ok(())
    }
    // TODO ADD CLASSPATH
    pub fn resolve_class_reference(this_loaded_classes: &mut HashMap<String, Rc<Class>>, reference: &str) -> Result<Rc<Class>, Error> {
        if !this_loaded_classes.contains_key(reference) {
            Self::load_class_file(this_loaded_classes, reference)?;
        }
        Ok(this_loaded_classes.get(reference).unwrap().clone())
    }
    pub fn resolve_class_reference_mut(this_loaded_classes: &mut HashMap<String, Rc< Class>>, reference: &str) -> Result<Rc<Class>, Error> {
        println!("Attemping to resolve {} mutably", reference);
        println!("Currently loaded classes: {:#?}", this_loaded_classes);
        Err(Error::NoClassDefFoundError(Opcode::ResolveClassRef))
    }
    // This is used for resolving references to Java internal classes (e.g. java.lang.String).
    pub fn resolve_internal_class_reference(&self) -> Result<Rc<Class>, &'static str> {
        Err("todo")
    }
    // Use this for checking that the class derived is above the given class in the heiriarchy. 
    pub fn resolve_with_derived_class(&self, reference: &str, derived: Rc<Class>) -> Result<Rc<Class>, &'static str> {
        Err("todo")
    }

    pub fn get_super(this_loaded_classes: &mut HashMap<String, Rc< Class>>, derived: Rc<Class>, level: usize) -> Option<Rc< Class>> {
        None
    }
    pub fn get_direct_super(this_loaded_classes: &mut HashMap<String, Rc< Class>>, derived: Rc<Class>) -> Option<Rc< Class>> {
        Self::get_super(this_loaded_classes, derived, 1)
    }
    
    pub fn current_thread(&self) -> &Thread {
        &self.m_threads[self.m_thread_index]
    }
    pub fn throw(&mut self, err: Error) {
        self.m_thrown_error = err;
    }
    // Should be replaced with a version that takes ownership of the strings instead of allocating.
    pub fn crash(&mut self, reason: &str, base_traceback: &str) {
        self.m_crash_info = Crash{has_crashed: true, 
                                  crash_reason: String::from(reason), 
                                  base_traceback: String::from(base_traceback)};       
    }
    #[inline] fn has_encoutered_error(&self) -> bool {
        // Used for checking whether to stop execution. 
        (self.m_thrown_error != Error::None) || self.m_crash_info.has_crashed   
    }
    // TODO: Wide needs to be a setting on a thread, so that we don't contaminate across threads.
    pub fn step(&mut self, step_size: usize) {
        // First, before running the cycle, we check if the thread is waiting on a monitor. 
        // If it is, then we attempt to own it and check the result. If we don't own it, we can't progress, so we just exit.
        {
            let current_thread_index = self.m_thread_index;
            let thread = access_macros::current_thread_mut!(self);
            if thread.current_monitor.is_some() {
                let mut monitor_rc = match &mut thread.current_monitor {
                    Some(mrc) => mrc,
                    None => unreachable!(),
                };
                let monitor = match Rc::get_mut(&mut monitor_rc) {
                    Some(m) => m,
                    None => {
                        self.m_thrown_error = Error::DoubleMultableReferenceToMonitor(Opcode::BlockedThread);
                        return;
                    },
                };
                let success = monitor.try_enter(current_thread_index);
                if !success {
                    // Move on to the next thread.
                    return;
                }
            }
        }
        for _ in 0..step_size {
            self.step1();
            if self.has_encoutered_error() | self.m_has_halted {
                break;
            }
        }
    }
    pub fn run(&mut self) {
        while !self.m_crash_info.has_crashed {
            self.step(self.m_step_size);
            if self.m_thrown_error != Error::None {
                if self.m_thrown_error == Error::Exception {
                    let err = self.handle_exception();
                    if err.is_err() {
                        // Set up a crash reason. this function should only crash if something really bad went wrong.
                    }
                }
                else {
                    // Some error has occured in our implementation.
                    // This just means that we should crash.
                    self.m_crash_info.has_crashed = true;
                    // TODO: Add crash reason.
                    self.m_crash_info.crash_reason = format!("{:#?}", self.m_thrown_error);
                }
            }
            if self.m_crash_info.has_crashed {
                println!("JVM Crashing due to error {}!", self.m_crash_info.crash_reason);
                // TODO: Implement stack unwinding.
                println!("Stack Trace:");
                for frame in &access_macros::current_thread_mut!(self).m_stack {
                    let current_class = &frame.rt_const_pool;
                    // TODO: Fix these unwrap calls.
                    println!("Method name: {}", current_class.cp_entry(frame.current_method.name_index).unwrap());
                    println!("Method descriptor: {}", current_class.cp_entry(frame.current_method.descriptor_index).unwrap());
                }
                return;
            }
            if self.m_has_halted {
                return;
            }
            self.m_thread_index += 1;
            self.m_thread_index %= self.m_threads.len();
        }
    }
    pub fn run_until_method_exit(&mut self) {
        // This function is used for calling methods while in an instructions. 
        // It is most often used for initializing instances of exceptions.
        let starting_call_stack_size = {
            let current_thread = access_macros::current_thread_mut!(self);
            current_thread.m_stack.len()
        };
        loop {
            self.step1();
            if self.has_encoutered_error() {
                break;
            }
            let current_thread = access_macros::current_thread_mut!(self);
            let current_call_stack_size = current_thread.m_stack.len();
            if current_call_stack_size < starting_call_stack_size {
                break;
            }
        }
    }
    // This class doesn't take a reference, because execution should be the end of a JVM.
    pub fn excecute(mut self) {
        if let Err(e) = self.setup_method_call_from_name_on_main("main", "([Ljava/lang/String;)V", true) {
            if e != Error::None {
                if e == Error::Exception {
                    let err = self.handle_exception();
                    if err.is_err() {
                        // Set up a crash reason. this function should only crash if something really bad went wrong.
                    }
                }
                else {
                    // Some error has occured in our implementation.
                    // This just means that we should crash.
                    self.m_crash_info.has_crashed = true;
                    // TODO: Add crash reason.
                    self.m_crash_info.crash_reason = format!("{:#?}", e);
                }
            }
            if self.m_crash_info.has_crashed {
                println!("JVM Crashing due to error {}", self.m_crash_info.crash_reason);
            }
        }
        self.run()
    }
}


impl<'a> JVM {
    pub fn step1(&mut self) {
        let wide: bool;
        let opcode = {
            let thread = self.current_thread();
            if thread.m_stack.len() == 0 {
                self.m_has_halted = true;
                return;
            }
            wide = thread.next_instruction_is_wide;
            let pc = thread.pc();
            thread.current_frame().current_method.code_at(pc)
        };
        if let Err(e) = opcode {
            self.m_thrown_error = e;
            return;
        }
        let op = opcode.unwrap();
        let err = match op {
            0 => self.nop(),
            1 => self.aconst_null(),
            2 => self.iconst_m1(),
            3 => self.iconst_0(),
            4 => self.iconst_1(),
            5 => self.iconst_2(),
            6 => self.iconst_3(),
            7 => self.iconst_4(),
            8 => self.iconst_5(),
            9 => self.lconst_0(),
            10 => self.lconst_1(),
            11 => self.fconst_0(),
            12 => self.fconst_1(),
            13 => self.fconst_2(),
            14 => self.dconst_0(),
            15 => self.dconst_1(),
            16 => self.bipush(),
            17 => self.sipush(),
            18 => self.ldc(),
            19 => self.ldc_w(),
            20 => self.ldc2_w(),
            21 => self.iload(wide),
            22 => self.lload(wide),
            23 => self.fload(wide),
            24 => self.dload(wide),
            25 => self.aload(wide),
            26 => self.iload_0(),
            27 => self.iload_1(),
            28 => self.iload_2(),
            29 => self.iload_3(),
            30 => self.lload_0(),
            31 => self.lload_1(),
            32 => self.lload_2(),
            33 => self.lload_3(),
            34 => self.fload_0(),
            35 => self.fload_1(),
            36 => self.fload_2(),
            37 => self.fload_3(),
            38 => self.dload_0(),
            39 => self.dload_1(),
            40 => self.dload_2(),
            41 => self.dload_3(),
            42 => self.aload_0(),
            43 => self.aload_1(),
            44 => self.aload_2(),
            45 => self.aload_3(),
            46 => self.iaload(),
            47 => self.laload(),
            48 => self.faload(),
            49 => self.daload(),
            50 => self.aaload(),
            51 => self.baload(),
            52 => self.caload(),
            53 => self.saload(),
            54 => self.istore(wide),
            55 => self.lstore(wide),
            56 => self.fstore(wide),
            57 => self.dstore(wide),
            58 => self.astore(wide),
            59 => self.istore_0(),
            60 => self.istore_1(),
            61 => self.istore_2(),
            62 => self.istore_3(),
            63 => self.lstore_0(),
            64 => self.lstore_1(),
            65 => self.lstore_2(),
            66 => self.lstore_3(),
            67 => self.fstore_0(),
            68 => self.fstore_1(),
            69 => self.fstore_2(),
            70 => self.fstore_3(),
            71 => self.dstore_0(),
            72 => self.dstore_1(),
            73 => self.dstore_2(),
            74 => self.dstore_3(),
            75 => self.astore_0(),
            76 => self.astore_1(),
            77 => self.astore_2(),
            78 => self.astore_3(),
            79 => self.iastore(),
            80 => self.lastore(),
            81 => self.fastore(),
            82 => self.dastore(),
            83 => self.aastore(),
            84 => self.bastore(),
            85 => self.castore(),
            86 => self.sastore(),
            87 => self.pop(),
            88 => self.pop2(),
            89 => self.dup(),
            90 => self.dup_x1(),
            91 => self.dup_x2(),
            92 => self.dup2(),
            93 => self.dup2_x1(),
            94 => self.dup2_x2(),
            95 => self.swap(),
            96 => self.iadd(),
            97 => self.ladd(),
            98 => self.fadd(),
            99 => self.dadd(),
            100 => self.isub(),
            101 => self.lsub(),
            102 => self.fsub(),
            103 => self.dsub(),
            104 => self.imul(),
            105 => self.lmul(),
            106 => self.fmul(),
            107 => self.dmul(),
            108 => self.idiv(),
            109 => self.ldiv(),
            110 => self.fdiv(),
            111 => self.ddiv(),
            112 => self.irem(),
            113 => self.lrem(),
            114 => self.frem(),
            115 => self.drem(),
            116 => self.ineg(),
            117 => self.lneg(),
            118 => self.fneg(),
            119 => self.dneg(),
            120 => self.ishl(),
            121 => self.lshl(),
            122 => self.ishr(),
            123 => self.lshr(),
            124 => self.iushr(),
            125 => self.lushr(),
            126 => self.iand(),
            127 => self.land(),
            128 => self.ior(),
            129 => self.lor(),
            130 => self.ixor(),
            131 => self.lxor(),
            132 => self.iinc(),
            133 => self.i2l(),
            134 => self.i2f(),
            135 => self.i2d(),
            136 => self.l2i(),
            137 => self.l2f(),
            138 => self.l2d(),
            139 => self.f2i(),
            140 => self.f2l(),
            141 => self.f2d(),
            142 => self.d2i(),
            143 => self.d2l(),
            144 => self.d2f(),
            145 => self.i2b(),
            146 => self.i2c(),
            147 => self.i2s(),
            148 => self.lcmp(),
            149 => self.fcmpl(),
            150 => self.fcmpg(),
            151 => self.dcmpl(),
            152 => self.dcmpg(),
            153 => self.ifeq(),
            154 => self.ifne(),
            155 => self.iflt(),
            156 => self.ifge(),
            157 => self.ifgt(),
            158 => self.ifle(),
            159 => self.if_icmpeq(),
            160 => self.if_icmpne(),
            161 => self.if_icmplt(),
            162 => self.if_icmpge(),
            163 => self.if_icmpgt(),
            164 => self.if_icmple(),
            165 => self.if_amcpeq(),
            166 => self.if_amcpne(),
            167 => self.goto(),
            168 => self.jsr(),
            169 => self.ret(),
            170 => self.tableswitch(),
            171 => self.lookupswitch(),
            172 => self.ireturn(),
            173 => self.lreturn(),
            174 => self.freturn(),
            175 => self.dreturn(),
            176 => self.areturn(),
            177 => self.return_(),
            178 => self.getstatic(),
            179 => self.putstatic(),
            180 => self.getfield(),
            181 => self.putfield(),
            182 => self.invokevirtual(),
            183 => self.invokespecial(),
            184 => self.invokestatic(),
            185 => self.invokeinterface(),
            186 => self.invokedynamic(),
            187 => self.new(),
            188 => self.newarray(),
            189 => self.anewarray(),
            190 => self.arraylength(),
            191 => self.athrow(),
            192 => self.checkcast(),
            193 => self.instanceof(),
            194 => self.monitorenter(),
            195 => self.monitorexit(),
            196 => {
                let thread = access_macros::current_thread_mut!(self);
                thread.next_instruction_is_wide = true;                
                Ok(())
            },
            197 => self.multianewarray(),
            198 => self.ifnull(),
            199 => self.ifnonnull(),
            200 => self.goto_w(),
            201 => self.jsr_w(),
            202 => self.breakpoint(),
            
            254 => self.impdep1(),
            255 => self.impdep2(),
            _ => {
                panic!("Error: Opcode {} not supported yet", op);
            },
        };
        if let Err(e) = err {
            self.m_thrown_error = e;
        }
    }
}

// This could be made more readable if we used the S, T, SC, and TC names like the jvm spec.
// https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-6.html#jvms-6.5.instanceof
impl JVM {
    pub fn check_class(this_loaded_classes: &mut HashMap<String, Rc<Class>>, object_desc: &str, class_desc: &str) -> Result<bool, Error> {
        match object_desc.as_bytes()[0] as char {
            'L' => {
                match class_desc.as_bytes()[0] as char {
                    'L' => {
                        // First, resolve class and object.
                        let object = access_macros::resolve_class_reference!(*this_loaded_classes, object_desc)?;
                        let class = access_macros::resolve_class_reference!(*this_loaded_classes, class_desc)?;
                        match class.is_interface() {
                            true => {
                                // If class is an interface, then object must implement class.
                                // We should add a short-circuit in the case that object also refers to an interface, because interfaces can't have interfaces,
                                for interface_index in object.interfaces() {
                                    let class_index = class.cp_entry(*interface_index)?.as_class()?;
                                    let ob_class_desc = class.cp_entry(*class_index)?.as_utf8()?;
                                    if ob_class_desc == class_desc {
                                        return Ok(true);
                                    }
                                }
                                Ok(false)
                            },
                            false => {
                                // If class is a class, then object must be a subclass.
                                let mut current_class = object;
                                while current_class.has_super() {
                                    if current_class == class {
                                        return Ok(true);
                                    }
                                    let super_name_index = current_class.cp_entry(current_class.super_index().unwrap())?.as_class()?;
                                    let super_name = current_class.cp_entry(*super_name_index)?.as_utf8()?;
                                    current_class = access_macros::resolve_class_reference!(*this_loaded_classes, super_name.as_str())?;
                                }
                                Ok(false)
                            }
                        }
                    },
                    '[' => Ok(false),
                    _ => return Err(Error::IllegalDescriptor),
                }
            },
            '[' => {
                match class_desc.as_bytes()[0] as char {
                    'L' => {
                        let class = access_macros::resolve_class_reference!(*this_loaded_classes, class_desc)?;
                        match class.is_interface() {
                            true => {
                                // If class is an interface, it has to be an interface implemented by arrays. 
                                // i don't know what those are, so for now, we just return false.
                                Ok(false)
                            },
                            false => {
                                // If class is a class, it must be object.
                                Ok(class_desc == "Ljava/lang/thread")
                            },
                        }
                    },
                    '[' => {
                        // In this case, both object and class are arrays. 
                        // This means that either the component types must both be the same primitive type, or they must be references castable by these rules
                        let object_component = &object_desc[1..];
                        let class_component = &class_desc[1..];
                        if (object_component.as_bytes()[0] as char == 'L') | (object_component.as_bytes()[0] as char == '[') {
                            if !((class_component.as_bytes()[0] as char == 'L') | (class_component.as_bytes()[0] as char == '[')) {
                                return Err(Error::IllegalDescriptor); // Either object and class are both primitive or both references.
                            }
                            return Self::check_class(this_loaded_classes, object_component, class_component);
                        }
                        if (class_component.as_bytes()[0] as char == 'L') | (class_component.as_bytes()[0] as char == '[') {
                            return Err(Error::IllegalDescriptor); // Either object and class are both primitive or both references.
                        }
                        Ok(object_component.as_bytes()[0] == class_component.as_bytes()[0])
                    },
                    _ => return Err(Error::IllegalDescriptor),
                }
            },
            _ => return Err(Error::IllegalDescriptor),
        }
    }   
}

impl<'a> JVM {
    // This functions contains lots of redundant checks that should be removed.
    pub fn handle_exception(&mut self) -> Result<(), Error> {
        {
            // This function doesn't actually check for any exception that could be thrown by athrow. 
            // athrow has to check its own exceptions, and create any that could occur.
            let loaded_classes = &mut self.m_loaded_classes;
            let thread = access_macros::current_thread_mut!(self);
            let current_pc = thread.m_pc;
            let exception = {
                let frame: &mut Frame = access_macros::current_frame_mut!(thread);
                let len = frame.op_stack.len();
                if len == 0 {
                    return Err(Error::StackUnderflow(Opcode::ExceptionHandle));
                }
                let exception_rc = frame.op_stack[len - 1].as_reference().unwrap(); // We should check the soundness of this cast before calling.
                let exception: &Object = match &*exception_rc { // This should also be ensured by the caller.
                    Reference::Array(_, _) | Reference::Interface(_, _) | Reference::Null => return Err(Error::IncorrectReferenceType(Opcode::ExceptionHandle)),
                    Reference::Object(o, _) => &o,
                };
                let code = match &frame.current_method.code {
                    Some(c) => c,
                    None => unreachable!(), // Should be unreachable, because methods that don't have code can't have exceptions anyway.
                };
                for ex_handler in &code.exception_table {
                    if (current_pc >= ex_handler.start_pc as usize) && (current_pc < ex_handler.end_pc as usize) {
                        let catches_this = {
                            if ex_handler.catch_type == 0 {
                                true // This catches all exceptions
                            }
                            else {
                                let current_class = frame.rt_const_pool.clone();
                                let catch_class_name_index = *current_class.cp_entry(ex_handler.catch_type)?.as_class()?;
                                let catch_class_name = current_class.cp_entry(catch_class_name_index)?.as_utf8()?;
                                let catch_class = access_macros::resolve_class_reference!(*loaded_classes, catch_class_name.as_str())?;
                                // Check if exception refers to catch_class or one of its subclasses
                                let exception_class = exception.m_class.clone();
                                let mut current_exception_class = exception_class;
                                let mut ret_flag = false;
                                while current_exception_class.has_super() {
                                    if current_exception_class == catch_class {
                                        ret_flag = true;
                                        break;
                                    }
                                    current_exception_class = access_macros::resolve_class_reference!(*loaded_classes, current_exception_class.super_name().unwrap())?;
                                }
                                ret_flag
                            }
                        };
                        if catches_this {
                            access_macros::set_pc!(thread, ex_handler.handler_pc as usize, frame.current_method.code()?.len())?;
                            // Discard all values except for the exception
                            let exception_val = frame.op_stack.pop().unwrap();
                            frame.op_stack.clear();
                            frame.op_stack.push(exception_val);
                            // TODO: Deal with synchronized functions.
                            return Ok(());
                        }
                    }
                }
                // If we're here, we haven't found an exception handler.
                // In this case, we give back the exception we had.
                frame.op_stack.pop().unwrap()
            };
            // If we found no exception handler, pass it down the call chain
            let _ = thread.m_stack.pop();
            if thread.m_stack.len() > 0 {
                // Continue down the call chain
                let frame: &mut Frame = access_macros::current_frame_mut!(thread);
                frame.op_stack.push(exception);
                return self.handle_exception();
            }
        }
        // If we got here, it means that we were at the last frame. 
        // In that case, we remove this thread.
        let _ = self.m_threads.remove(self.m_thread_index);
        Ok(())

    }    
    pub fn setup_method_call_from_name(&mut self, name: &str, descriptor: &str, mut current_class: Rc< Class>, is_static: bool)  -> Result<(), Error> {
        let mut method_to_call = None; 
        {
            let mut found = false;
            let loaded_classes = &mut self.m_loaded_classes;
            while current_class.has_super() && !found {
                for method in current_class.methods() {
                    // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.3
                    // We still need to check for signature polymorphic functions.
                    let method_descriptor = current_class.cp_entry(method.descriptor_index)?.as_utf8()?;
                    if method_descriptor != descriptor {
                        continue;
                    }
                    let method_name = current_class.cp_entry(method.name_index)?.as_utf8()?;
                    if method_name == name {
                        method_to_call = Some(method.clone());
                        found = true;
                        break;
                    }
                }   
                // Recurse up the inheritance tree.
                if !found { 
                    current_class = access_macros::resolve_class_reference!(self.m_loaded_classes, current_class.super_name().unwrap())?; 
                }
                
            }
            // TODO: Search Superinterfaces of c.
            if !found {
                return Err(Error::NoSuchMethodError(Opcode::MethodInvoke));
            }
        };
        // Call into the actual setup method
        self.setup_method_call(&method_to_call.unwrap(), current_class, is_static,)
    }
    pub fn setup_method_call_from_name_on_main(&mut self, name: &str, descriptor: &str, is_static: bool)  -> Result<(), Error> {
        let main_class_name = &self.m_main_class_name;
        let mut current_class = access_macros::resolve_class_reference!(self.m_loaded_classes, main_class_name)?;
        let mut method_to_call = None; 
        {
            let mut found = false;
            while current_class.has_super() && !found {
                for method in current_class.methods() {
                    // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.3
                    // We still need to check for signature polymorphic functions.
                    let method_descriptor = current_class.cp_entry(method.descriptor_index)?.as_utf8()?;
                    if method_descriptor != descriptor {
                        continue;
                    }
                    let method_name = current_class.cp_entry(method.name_index)?.as_utf8()?;
                    if method_name == name {
                        method_to_call = Some(method.clone());
                        found = true;
                        break;
                    }
                }   
                // Recurse up the inheritance tree.
                if !found { 
                    current_class = access_macros::resolve_class_reference!(self.m_loaded_classes, current_class.super_name().unwrap())?; 
                }
                
            }
            // TODO: Search Superinterfaces of c..
            if !found {
                return Err(Error::NoSuchMethodError(Opcode::MethodInvoke));
            }
        };
        // Call into the actual setup method
        self.setup_method_call(&method_to_call.unwrap(), current_class, is_static)
    }
    pub fn setup_method_call(&mut self, method: &MethodInfo, c: Rc< Class>, is_static: bool) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let mut new_frame = Frame::new(c.clone(), method.clone());
        // Fill out the local variables.
        // There's an error here with accessing the method descriptor
        let mut descriptor: &str = c.cp_entry(method.descriptor_index)?.as_utf8()?;
        descriptor = &descriptor[0..descriptor.find(")").unwrap()]; // Skip past the return value
        descriptor = &descriptor[1..]; // Skip the beginning paranthesis.
        let locals = &mut new_frame.local_variables;
        while descriptor.len() > 0 {
            let mut index = descriptor.len() - 1;
            if &descriptor[index..] == ";" {
                index = descriptor.rfind('L').unwrap();
            }
            if &descriptor[index - 1..index] == "[" {
                // There is a better way of doing this, FIXME.
                descriptor = &descriptor[..index];
                index -= 1;
            }
            match &descriptor[index..index + 1] {
                "B" => {
                    let current_frame = access_macros::current_frame_mut!(thread);
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_int(val)?;
                    locals.push(VarValue::Byte(inner_value));
                    descriptor = &descriptor[..index];
                },
                "C" => {
                    let current_frame = access_macros::current_frame_mut!(thread);
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_int(val)?;
                    locals.push(VarValue::Char(inner_value));
                    descriptor = &descriptor[..index];
                },
                "D" => {
                    let current_frame = access_macros::current_frame_mut!(thread);
                    locals.push(VarValue::DoubleHighBytes);
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_double(val)?;
                    locals.push(VarValue::Double(inner_value));
                    descriptor = &descriptor[..index];
                },
                "F" => {
                    let current_frame = access_macros::current_frame_mut!(thread);
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_float(val)?;
                    locals.push(VarValue::Float(inner_value));
                    descriptor = &descriptor[..index];
                },
                "I" => {
                    let current_frame = access_macros::current_frame_mut!(thread);
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_int(val)?;
                    locals.push(VarValue::Int(inner_value));
                    descriptor = &descriptor[..index];
                },
                "J" => {
                    let current_frame = access_macros::current_frame_mut!(thread);
                    locals.push(VarValue::LongHighBytes);
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_long(val)?;
                    locals.push(VarValue::Long(inner_value));
                    descriptor = &descriptor[..index];
                },
                "S" => {
                    let current_frame = access_macros::current_frame_mut!(thread);
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_int(val)?;
                    locals.push(VarValue::Short(inner_value));
                    descriptor = &descriptor[..index];
                },
                "Z" => {
                    let current_frame = access_macros::current_frame_mut!(thread);
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_int(val)?;
                    locals.push(VarValue::Byte(inner_value & 1));
                    descriptor = &descriptor[..index];
                },
                "L" => {
                    let current_frame = access_macros::current_frame_mut!(thread);
                    // We could check the class type and make sure it matches up with the expected type, but that's not required by the JVM Spec, so for now we won't
                    let val = match current_frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                    };
                    let inner_value = Value::to_reference(val)?;
                    locals.push(VarValue::Reference(inner_value));
                    descriptor = &descriptor[..index];
                },
                "[" => {
                    let val = {
                        // If this code is the method "main", then we have to add the args manually
                        if thread.m_stack.len() > 0 {
                            let current_frame = access_macros::current_frame_mut!(thread);
                        // We could check the class type and make sure it matches up with the expected type, but that's not required by the JVM Spec, so for now we won't
                            match current_frame.op_stack.pop() {
                                Some(v) => v,
                                None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                            }
                        }
                        else {
                            if c.name() != self.m_main_class_name {
                                panic!();
                            }
                            else {
                                // TODO: Add actual arguments.
                                let args = Array::new_ref(0, String::from("Ljava/lang/String;"));
                                let args_ref = Reference::Array(args, Rc::new(Monitor::new()));
                                Value::Reference(Rc::new(args_ref))
                            }
                        }
                    };
                    let inner_value = Value::to_reference(val)?;
                    locals.push(VarValue::Reference(inner_value));
                    descriptor = &descriptor[..index];
                }
                _ => return Err(Error::IllegalDescriptor),
            }
        }
        if !is_static {
            let current_frame = access_macros::current_frame_mut!(thread);
            let objectref = match current_frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
            };
            let inner_ref = Value::to_reference(objectref)?;
            locals.push(VarValue::Reference(inner_ref));
        }
        locals.reverse(); // We have to push the variables in reverse order, so we correct it after.
        thread.push_frame(new_frame);
        Ok(())
    }
    pub fn execute_native(&mut self, method: &MethodInfo    ) -> Result<(), Error> {
        Err(Error::UnsatisfiedLinkError(Opcode::MethodInvoke))
    }
}