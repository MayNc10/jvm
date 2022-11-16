use crate::class::classfile::ClassFile;
use crate::reference::array::Array;
use crate::{access_macros, class};
use crate::errorcodes::{Error, Opcode};
use crate::class::{Class, classfile::MethodInfo};
use crate::frame::Frame;
use crate::reference::{Reference, Monitor};
use crate::thread::Thread;
use crate::value::{Value, VarValue};

use core::panic;
use std::collections::HashMap;
use std::fs::{File, self};
use std::io::Read;
use std::option::Option; 
use std::rc::Rc;
use std::result::Result;
use std::string::String;
use std::time::Instant;
use std::vec::Vec;

use self::instructions::Instruction;

// Just useful for code readability
mod operations;
pub mod instructions;
pub mod settings;

const STEP_SIZE: usize = 10;

pub struct Crash {
    has_crashed: bool,
    crash_reason: String,
    _base_traceback: String,
} 

pub struct JVM {
    pub m_threads: Vec<Thread>,
    pub m_loaded_classes: HashMap<String, Rc<dyn Class>>,
    m_thrown_error: Error,
    m_crash_info: Crash,
    pub m_thread_index: usize,
    m_step_size: usize,
    m_has_halted: bool,
    pub m_main_class_name: String,
    m_flags: u8,
    pub start_time: Instant,
    class_path: Option<String>,
}

impl JVM {
    pub fn new_jvm(n: String, flags: u8, class_path: Option<String>) -> JVM {
        JVM {
            m_threads: vec![Thread::new()
            ],
            m_loaded_classes: HashMap::new(),
            m_thrown_error: Error::None,
            m_crash_info: Crash{ has_crashed: false, 
                crash_reason: String::from(""), 
                _base_traceback: String::from("")},
            m_thread_index: 0,
            m_step_size: STEP_SIZE,
            m_has_halted: false,
            m_main_class_name: n,
            m_flags: flags,
            start_time: Instant::now(),
            class_path,
        }
    }
    pub fn new_with_step_size(n: String, step_size: usize, flags: u8, class_path: Option<String>) -> JVM {
        JVM {
            m_threads: Vec::new(),
            m_loaded_classes: HashMap::new(),
            m_thrown_error: Error::None,
            m_crash_info: Crash{ has_crashed: false, 
                crash_reason: String::from(""), 
                _base_traceback: String::from("")},
            m_thread_index: 0,
            m_step_size: step_size,
            m_has_halted: false,
            m_main_class_name: n,
            m_flags: flags,
            start_time: Instant::now(),
            class_path,
        }
    }
    pub fn new_with_main_class(c: ClassFile, code_bytes: Vec<Vec<u8>>, flags: u8, class_path: Option<String>) -> Result<JVM, Error> {
        let mut jvm = Self::new_jvm(String::from(c.name()), flags, class_path);

        let name = String::from(c.name());
        let class = class::new_class(c, &mut jvm)?;
        jvm.m_loaded_classes.insert(name, class.clone());

        // Init the class file.
        unsafe { Rc::get_mut_unchecked(&mut class.get_class_file()) }.init_code(code_bytes, &mut jvm)?;
        
        // After adding the class, we initialize the class
        // We should use this error instead of ignoring it.
        let mut found_clinit = true;
        if let Err(e) = jvm.setup_method_call_from_name_on_main("<clinit>", "()V", true) {
            if e != Error::NoSuchMethodError(Opcode::MethodInvoke) {
                println!("{e:#?}");
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
    pub fn load_class_file(&mut self, path: &str) -> Result<(), Error> {    
        let mut resolved_path = String::from(path);
        if &path[0..4] == "java" {
            //TODO: Add java class path
            // For now we just hardcode the directory, in the future we will have to change that
            let mut temp_str = match self.class_path.as_ref() {
                Some(s) => s.clone(),
                None => String::from("./classlibs/17/java.base/"),
            };
            temp_str.push_str(&resolved_path);
            resolved_path = temp_str;
        }
        resolved_path.push_str(".class");
        let mut f = match File::open(resolved_path.as_str()) {
            Ok(file) => file,
            Err(_) => {
                println!("Path: {path}");
                return Err(Error::NoClassDefFoundError(Opcode::ClassLoad));
            },
        };
        let metadata = fs::metadata(&resolved_path).unwrap();
        let size = if metadata.len() % 8 == 0 {metadata.len() / 8} else {metadata.len() / 8 + 1};
        let buffer: Vec<u64> = vec![0; size as usize];
        let (file, code) = unsafe {
            let buf_bytes = std::slice::from_raw_parts_mut(buffer.as_ptr() as *mut u8, buffer.len() * std::mem::size_of::<u64>());
            f.read(buf_bytes).unwrap();
            class::classfile::ClassFile::new(buf_bytes)?
        };
        let c = class::new_class(file, self)?;
        // Adding the class to the map here seems a bit weird, but if we don't we overflow the stack.
        self.m_loaded_classes.insert(String::from(c.get_class_file().name()), Rc::clone(&c)); 
        // init the code here to prevent endless recursion
        unsafe { Rc::get_mut_unchecked(&mut c.get_class_file())}.init_code(code, self)?;
        // clinit if a class has it.
        let mut found_clinit = true;
        if let Err(e) = self.setup_method_call_from_name_on_main("<clinit>", "()V", true) {
            if e != Error::NoSuchMethodError(Opcode::MethodInvoke) {
                return Err(e);
            }
            found_clinit = false;
        }
        if found_clinit {
            self.run_until_method_exit();
        }
        Ok(())
    }
    // TODO ADD CLASSPATH
    pub fn resolve_class_reference(&mut self, reference: &str) -> Result<Rc<dyn Class>, Error> {
        if !self.m_loaded_classes.contains_key(reference) {
            self.load_class_file(reference)?;
        }
        Ok(self.m_loaded_classes.get(reference).unwrap().clone())
    }
    // Use this for checking that the class derived is above the given class in the heiriarchy. 
    pub fn resolve_with_derived_class(&self, _reference: &str, _derived: Rc<dyn Class>) -> Result<Rc<dyn Class>, &'static str> {
        Err("todo")
    }

    pub fn get_super(_this_loaded_classes: &mut HashMap<String, Rc<dyn Class>>, _derived: Rc<dyn Class>, _level: usize) -> Option<Rc<dyn Class>> {
        None
    }
    pub fn get_direct_super(this_loaded_classes: &mut HashMap<String, Rc<dyn Class>>, derived: Rc<dyn Class>) -> Option<Rc<dyn Class>> {
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
                                  _base_traceback: String::from(base_traceback)};       
    }
    fn has_encoutered_error(&self) -> bool {
        // Used for checking whether to stop execution. 
        (self.m_thrown_error != Error::None) || self.m_crash_info.has_crashed   
    }
    pub fn step(&mut self, step_size: usize) {
        // First, before running the cycle, we check if the thread is waiting on a monitor. 
        // If it is, then we attempt to own it and check the result. If we don't own it, we can't progress, so we just exit.
        {
            let current_thread_index = self.m_thread_index;
            let thread = access_macros::current_thread_mut!(self);
            if thread.current_monitor.is_some() {
                let monitor_rc = match &mut thread.current_monitor {
                    Some(mrc) => mrc,
                    None => unreachable!(),
                };
                let monitor = match Rc::get_mut(monitor_rc) {
                    Some(m) => m,
                    None => {
                        self.m_thrown_error = Error::DoubleMutableReferenceToMonitor(Opcode::BlockedThread);
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
            let old_pc = self.current_thread().pc();
            let old_frame_num = self.current_thread().m_stack.len();
            self.step1();
            if self.has_encoutered_error() | self.m_has_halted {
                break;
            }
            let thread = current_thread_mut!(self);
            if thread.m_stack.len() == 0 {
                // Done with code, exit.
                self.m_threads.remove(self.m_thread_index);
                return;
            }
            if (self.current_thread().pc() == old_pc && self.current_thread().m_stack.len() == old_frame_num) || self.current_thread().m_stack.len() < old_frame_num {
                // Means we haven't made any jumps, increment. 
                // Technically this allows for jumps down. This is intentional, because otherwise we would jump down to the instruction that jumped us up,
                // creating an infinite loop.
                // The only way this breaks is if an instruction jumps to itself.
                // FIXME: Deal with these types of infinite loops.
                let thread = current_thread_mut!(self);
                thread.inc_pc(1); // IMPORTANT: Account for this in instructions.
            }
        }
    }
    pub fn run(&mut self) {
        while !self.m_crash_info.has_crashed {
            let old_num_threads = self.m_threads.len();
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
                println!("JVM Crashing due to error {}", self.m_crash_info.crash_reason);
                if (self.m_flags & settings::SHOULD_BACKTRACE) > 0 {
                    println!("Backtrace:");
                    for frame in access_macros::current_thread_mut!(self).m_stack.iter().rev() {
                        let current_class = &frame.rt_const_pool;
                        // TODO: Fix these unwrap calls.
                        println!("Method name: {}, Method descriptor: {}, Method class: {}", current_class.get_class_file().cp_entry(frame.current_method.name_index).unwrap().as_utf8().unwrap(),
                        current_class.get_class_file().cp_entry(frame.current_method.descriptor_index).unwrap().as_utf8().unwrap(), current_class.get_class_file().name());
                        println!("Local variables:");
                        for local in frame.local_variables.iter().rev() {
                            println!("  {local:#?}");
                        }
                        println!("Operand stack:");
                        for operand in frame.op_stack.iter().rev() {
                            println!("  {operand:#?}");
                        }
                        println!("Current pc and instruction: {}, {}", frame.pc, frame.current_method.code_at(frame.pc).unwrap());
                        println!("Current code: {}", frame.current_method.code.as_ref().unwrap());
                        //println!("Current class: {}", current_class.get_class_file());
                    }
                }
                return;
            }
            if self.m_has_halted {
                return;
            }
            if self.m_threads.len() < old_num_threads {
                // Don't inc idx and possibly exit
                if self.m_threads.len() == 0 {
                    return;
                }
            } else {
                self.m_thread_index += 1;
            }
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
                    self.m_crash_info.crash_reason = format!("{e:#?}");
                }
            }
            if self.m_crash_info.has_crashed {
                println!("JVM Crashing due to error {}", self.m_crash_info.crash_reason);
            }
        }
        self.run()
    }
}


impl JVM {
    pub fn step1(&mut self) {
        let (opcode, pc) = {
            let thread = current_thread_mut!(self);
            if thread.m_stack.is_empty() {
                self.m_has_halted = true;
                return;
            }
            let pc = thread.pc();
            let frame = current_frame_mut!(thread);
            (frame.current_method.code_at_mut(pc), pc)
        };
        if let Err(e) = opcode {
            self.m_thrown_error = e;
            return;
        }
        let op = opcode.unwrap() as *mut Box<dyn Instruction>;
        //println!("Executing {}: {:?}", pc, unsafe{ &*op } );
        /* 
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
        */
        let err = unsafe {
            // I can't find a way to express to Rust what I want to do here, so we have to use unsafe. 
            // Essentially, the op is always 'within' the JVM and so this will always be a double borrow of self.
            // I don't think there is a way to guarantee to Rust that op.execute will never delete the op through accessing the class that owns it. 
            (*op).execute(self)
        };
        if let Err(e) = err {
            self.m_thrown_error = e;
        }
    }
}

// This could be made more readable if we used the S, T, SC, and TC names like the jvm spec.
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.instanceof
impl JVM {
    pub fn check_class(&mut self, object_desc: &str, class_desc: &str) -> Result<bool, Error> {
        match object_desc.as_bytes()[0] as char {
            'L' => {
                match class_desc.as_bytes()[0] as char {
                    'L' => {
                        // First, resolve class and object.
                        let object_class = self.resolve_class_reference(object_desc)?;
                        let class = self.resolve_class_reference(class_desc)?;
                        let object_class_file = object_class.get_class_file();
                        let class_file = class.get_class_file();
                        match class_file.is_interface() {
                            true => {
                                // If class is an interface, then object must implement class.
                                // We should add a short-circuit in the case that object also refers to an interface, because interfaces can't have interfaces,
                                for interface_index in object_class_file.interfaces() {
                                    let class_index = class_file.cp_entry(*interface_index)?.as_class()?;
                                    let ob_class_desc = class_file.cp_entry(*class_index)?.as_utf8()?;
                                    if ob_class_desc == class_desc {
                                        return Ok(true);
                                    }
                                }
                                Ok(false)
                            },
                            false => {
                                // If class is a class, then object must be a subclass.
                                let mut current_class = object_class;
                                let mut current_class_file = object_class_file;
                                while current_class.get_class_file().has_super() {
                                    #[allow(clippy::vtable_address_comparisons)]
                                    if Rc::ptr_eq(&current_class, &class) {
                                        return Ok(true);
                                    }
                                    let super_name_index = current_class_file.cp_entry(current_class.get_class_file().super_index().unwrap())?.as_class()?;
                                    let super_name = current_class_file.cp_entry(*super_name_index)?.as_utf8()?;
                                    current_class = self.resolve_class_reference(super_name.as_str())?;
                                    current_class_file = current_class.get_class_file();
                                }
                                Ok(false)
                            }
                        }
                    },
                    '[' => Ok(false),
                    _ => Err(Error::IllegalDescriptor),
                }
            },
            '[' => {
                match class_desc.as_bytes()[0] as char {
                    'L' => {
                        let class = self.resolve_class_reference(class_desc)?;
                        match class.get_class_file().is_interface() {
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
                            return self.check_class(object_component, class_component);
                        }
                        if (class_component.as_bytes()[0] as char == 'L') | (class_component.as_bytes()[0] as char == '[') {
                            return Err(Error::IllegalDescriptor); // Either object and class are both primitive or both references.
                        }
                        Ok(object_component.as_bytes()[0] == class_component.as_bytes()[0])
                    },
                    _ => Err(Error::IllegalDescriptor),
                }
            },
            _ => Err(Error::IllegalDescriptor),
        }
    }   
}

impl JVM {
    // This functions contains lots of redundant checks that should be removed.
    pub fn handle_exception(&mut self) -> Result<(), Error> {
        {
            // This function doesn't actually check for any exception that could be thrown by athrow. 
            // athrow has to check its own exceptions, and create any that could occur.
            let thread = access_macros::current_thread_mut!(self);
            let current_pc = thread.pc();
            let exception = {
                let frame: &mut Frame = access_macros::current_frame_mut!(thread);
                let len = frame.op_stack.len();
                if len == 0 {
                    return Err(Error::StackUnderflow(Opcode::ExceptionHandle));
                }
                let exception_rc = frame.op_stack[len - 1].as_reference().unwrap().clone(); // We should check the soundness of this cast before calling.
                let exception = match exception_rc { // This should also be ensured by the caller.
                    Reference::Array(_, _) | Reference::Interface(_, _) | Reference::Null => 
                    return Err(Error::IncorrectReferenceType(Opcode::ExceptionHandle)),
                    Reference::Object(o, _) => o,
                };
                let code = match frame.current_method.code.clone() {
                    Some(c) => c,
                    None => unreachable!(), // Should be unreachable, because methods that don't have code can't have exceptions anyway.
                };
                let current_class = frame.rt_const_pool.clone();                
                for ex_handler in &code.exception_table {
                    if (current_pc >= ex_handler.start_pc as usize) && (current_pc < ex_handler.end_pc as usize) {
                        let catches_this = {
                            if ex_handler.catch_type == 0 {
                                true // This catches all exceptions
                            }
                            else {               
                                let current_class_file = current_class.get_class_file();            
                                let catch_class_name_index = *current_class_file.cp_entry(ex_handler.catch_type)?.as_class()?;
                                let catch_class_name = current_class_file.cp_entry(catch_class_name_index)?.as_utf8()?;
                                let catch_class = self.resolve_class_reference(catch_class_name.as_str())?;
                                // Check if exception refers to catch_class or one of its subclasses
                                let exception_class = exception.class().clone();
                                let mut current_exception_class = exception_class;
                                let mut ret_flag = false;
                                while current_exception_class.get_class_file().has_super() {
                                    #[allow(clippy::vtable_address_comparisons)]
                                    if Rc::ptr_eq(&current_exception_class, &catch_class) {
                                        ret_flag = true;
                                        break;
                                    }
                                    current_exception_class = self.resolve_class_reference(current_exception_class.get_class_file().super_name().unwrap())?;
                                }
                                ret_flag
                            }
                        };
                        if catches_this {
                            // Reaquire the frame
                            let thread = access_macros::current_thread_mut!(self);
                            let frame = access_macros::current_frame_mut!(thread);
                            frame.pc = ex_handler.handler_pc as usize;
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
                let thread = access_macros::current_thread_mut!(self);
                let frame = access_macros::current_frame_mut!(thread);
                frame.op_stack.pop().unwrap()
            };
            // If we found no exception handler, pass it down the call chain
            let thread = access_macros::current_thread_mut!(self);
            let _ = thread.m_stack.pop();
            if !thread.m_stack.is_empty() {
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
    pub fn setup_method_call_from_name(&mut self, name: &str, descriptor: &str, mut current_class: Rc<dyn Class>, is_static: bool)  -> Result<(), Error> {
        let mut method_to_call = None; 
        {
            let mut found = false;
            let mut current_class_file = current_class.get_class_file();
            while current_class_file.has_super() && !found {
                for method in current_class.get_class_file().methods() {
                    // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.3
                    // We still need to check for signature polymorphic functions.
                    let method_descriptor = current_class_file.cp_entry(method.descriptor_index)?.as_utf8()?;
                    if method_descriptor != descriptor {
                        continue;
                    }
                    let method_name = current_class_file.cp_entry(method.name_index)?.as_utf8()?;
                    if method_name == name {
                        method_to_call = Some(method.clone());
                        found = true;
                        break;
                    }
                }   
                // Recurse up the inheritance tree.
                if !found { 
                    current_class = self.resolve_class_reference(current_class.get_class_file().super_name().unwrap())?;
                    current_class_file = current_class.get_class_file();
                }
                
            }
            // TODO: Search Superinterfaces of c.
            if !found {
                return Err(Error::NoSuchMethodError(Opcode::MethodInvoke));
            }
        };
        // Call into the actual setup method
        self.setup_method_call(&method_to_call.unwrap(), current_class, is_static)
    }
    pub fn setup_method_call_from_name_on_main(&mut self, name: &str, descriptor: &str, is_static: bool)  -> Result<(), Error> {
        let main_class_name = self.m_main_class_name.clone();
        let mut current_class = self.resolve_class_reference(&main_class_name)?;
        let mut current_class_file = current_class.get_class_file();
        let mut method_to_call = None; 
        {
            let mut found = false;
            while current_class_file.has_super() && !found {
                for method in current_class.get_class_file().methods() {
                    // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.3
                    // We still need to check for signature polymorphic functions.
                    let method_descriptor = current_class_file.cp_entry(method.descriptor_index)?.as_utf8()?;
                    if method_descriptor != descriptor {
                        continue;
                    }
                    let method_name = current_class_file.cp_entry(method.name_index)?.as_utf8()?;
                    if method_name == name {
                        method_to_call = Some(method.clone());
                        found = true;
                        break;
                    }
                }   
                // Recurse up the inheritance tree.
                if !found { 
                    current_class = self.resolve_class_reference(current_class.get_class_file().super_name().unwrap())?;
                    current_class_file = current_class.get_class_file();
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
    pub fn setup_method_call(&mut self, method: &MethodInfo, c: Rc<dyn Class>, is_static: bool) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let mut new_frame = Frame::new(c.clone(), method.clone());
        // Fill out the local variables.
        let c_file = c.get_class_file();
        let mut descriptor: &str = c_file.cp_entry(method.descriptor_index)?.as_utf8()?;
        descriptor = &descriptor[0..descriptor.find(')').unwrap()]; // Skip past the return value
        descriptor = &descriptor[1..]; // Skip the beginning parenthesis.
        let locals = &mut new_frame.local_variables;
        while !descriptor.is_empty() {
            let mut index = descriptor.len() - 1;
            if &descriptor[index..] == ";" {
                index = descriptor.rfind('L').unwrap();
            }
            if (index > 0) && &descriptor[index - 1..index] == "[" {
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
                        if !thread.m_stack.is_empty() {
                            let current_frame = access_macros::current_frame_mut!(thread);
                        // We could check the class type and make sure it matches up with the expected type, but that's not required by the JVM Spec, so for now we won't
                            match current_frame.op_stack.pop() {
                                Some(v) => v,
                                None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                            }
                        }
                        else if c.get_class_file().name() != self.m_main_class_name {
                                panic!();
                            }
                        else {
                            // TODO: Add actual arguments.
                            let args = Array::new_ref(0, String::from("Ljava/lang/String;"));
                            let args_ref = Reference::Array(Rc::new(args), Rc::new(Monitor::new()));
                            Value::Reference(args_ref)
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
    pub fn execute_native_from_name(&mut self, name: &str, descriptor: &str, mut current_class: Rc<dyn Class>) -> Result<(), Error> {
        let mut method_to_call = None; 
        {
            let mut found = false;
            let mut current_class_file = current_class.get_class_file();
            while current_class_file.has_super() && !found {
                for method in current_class.get_class_file().methods() {
                    // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.3
                    // We still need to check for signature polymorphic functions.
                    let method_descriptor = current_class_file.cp_entry(method.descriptor_index)?.as_utf8()?;
                    if method_descriptor != descriptor {
                        continue;
                    }
                    let method_name = current_class_file.cp_entry(method.name_index)?.as_utf8()?;
                    if method_name == name {
                        method_to_call = Some(method.clone());
                        found = true;
                        break;
                    }
                }   
                // Recurse up the inheritance tree.
                if !found { 
                    current_class = self.resolve_class_reference(current_class.get_class_file().super_name().unwrap())?; 
                    current_class_file = current_class.get_class_file();
                }
                
            }
            // TODO: Search Superinterfaces of c.
            if !found {
                return Err(Error::NoSuchMethodError(Opcode::MethodInvoke));
            }
        };
        // Call into the actual setup method
        self.execute_native(&method_to_call.unwrap(), current_class)
    }
    pub fn execute_native(&mut self, method: &MethodInfo, current_class: Rc<dyn Class>) -> Result<(), Error> {
        let current_class_file = current_class.get_class_file();
        let mname = current_class_file.cp_entry(method.name_index)?.as_utf8()?;
        let _mdesc = current_class_file.cp_entry(method.descriptor_index)?.as_utf8()?;
        let cname = current_class_file.name();
        #[allow(clippy::match_single_binding)]
        match cname {
            _ => Err(Error::UnsatisfiedLinkError(Opcode::MethodInvoke, mname.clone()))
        }    
    }
    pub fn execute_on_object(&mut self, method: &MethodInfo, current_class: Rc<dyn Class>) -> Result<(), Error> {
        let num_args = method.num_args(&current_class.get_class_file())?;
        let thread = current_thread_mut!(self); let frame = current_frame_mut!(thread);
        let mut obj_ref = frame.op_stack[frame.op_stack.len() - num_args - 1].as_reference()?;
        let mut obj_ref = Rc::clone(obj_ref.as_object_mut().unwrap());
        let obj = unsafe { Rc::get_mut_unchecked(&mut obj_ref)};
        obj.exec_method(Rc::clone(&current_class), self, method)?;
        // Get rid of remaining object ref
        let thread = current_thread_mut!(self); let frame = current_frame_mut!(thread);
        if method.returns_void(&current_class.get_class_file())? {
            frame.op_stack.pop();
        }
        else {
            let ret_val = frame.op_stack.pop().unwrap();
            frame.op_stack.pop();
            frame.op_stack.push(ret_val);
        }
        Ok(())
    }
}

impl JVM {
    pub fn parse_descriptor(_desc: &str) -> Result<(&[&str], &str), Error> {
        panic!("todo");
    }
    pub fn box_primitive_name(&mut self, sym: &str) -> Result<&'static str, Error> {
        match sym {
            "B" => Ok("java/lang/Byte"),
            "C" => Ok("java/lang/Char"),
            "D" => Ok("java/lang/Double"),
            "F" => Ok("java/lang/Float"),
            "I" => Ok("java/lang/Int"),
            "J" => Ok("java/lang/Long"),
            "S" => Ok("java/lang/Short"),
            "Z" => Ok("java/lang/Boolean"),
            _ => Err(Error::IllegalDescriptor),
        }
    }
    pub fn gen_class_obj(&mut self, name: &str) -> Result<(), Error> {
        // We have to create an instance of the class. 
        // To do this we call the private constructor with the classloader and the array component type
        // Since we don't have class loaders, this is always null for now
        let loader = Value::Reference(Reference::Null);
        let thread = access_macros::current_thread_mut!(self);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(loader);
        if &name[0..1] == "[" {
            let mut idx = 1;
            while &name[idx..idx+1] == "[" {
                idx += 1;
            }
            if &name[idx..idx+1] == "L" {
                let comp_name = &name[idx + 1..];
                self.gen_class_obj(comp_name)?; // Puts the comp type on the stack
            }
            else {
                // NOTE: This might be incorrect behavior. 
                // It's possible that when we encounter a primitive type, we shouldn't box it into a class
                // For now we do, it makes things easier
                let boxed_name = self.box_primitive_name(&name[idx..idx+1])?;
                self.gen_class_obj(boxed_name)?;
            }
        }
        else { 
            frame.op_stack.push(Value::Reference(Reference::Null));
        }
        // This may not work right, idk how exactly it should work. This will probably need explicit support from the vm in the future.
        let class_class = self.resolve_class_reference("java/lang/Class")?;
        self.setup_method_call_from_name("<init>", "(Ljava/lang/ClassLoader;Ljava/lang/Class;)V", class_class, true)?;
        self.run_until_method_exit();
        Ok(())
    }
}