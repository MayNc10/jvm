use colored::Colorize;

use std::{collections::HashMap, rc::Rc};

use crate::attributes::code::stack_map_table;
use crate::llvm::valuemarker::ValueMarker;
use crate::{constant_pool::{NameAndType, Entry}, value::{Value, VarValue}, errorcodes::Opcode, 
            flags, reference::{Reference, array::Array, Monitor}, access_macros, frame::Frame};

use super::*;

pub struct CustomClass {
    class_file: Rc<classfile::ClassFile>,
    static_fields: HashMap<NameAndType, Rc<Value<dyn Class, dyn Object>>>, 
    #[cfg(not(target_family = "wasm"))]
    context: &'static Context,
    #[cfg(not(target_family = "wasm"))]
    module: Module<'static>,
    #[cfg(not(target_family = "wasm"))]
    builder: Builder<'static>,
    #[cfg(not(target_family = "wasm"))]
    execution_engine: ExecutionEngine<'static>,
}

impl Class for CustomClass {
    // We could use a different type than NameAndType for the &Strings, but this is simpler and terribly slow.
    fn new(file: classfile::ClassFile, jvm: &mut JVM) -> Result<Self, Error> where Self : Sized {
        let mut static_fields = HashMap::new();
        // Fill in static fields.
        for field in &file.fields {
            if (field.access_flags.flags & flags::field::ACC_STATIC) > 0 {
                let static_field_name = file.constant_pool[field.name_index as usize - 1].as_utf8()?.clone();
                let static_field_desc = file.constant_pool[field.descriptor_index as usize - 1].as_utf8()?.clone();
                let name_and_type = NameAndType {
                    name: static_field_name,
                    descriptor: static_field_desc,
                };
                let value = {
                    if field.constant_value.is_some() {
                        match file.constant_pool[field.constant_value.unwrap() as usize - 1] {
                            Entry::Integer(i) => match &name_and_type.descriptor[0..1] {
                                "B" | "Z" => Value::Byte(i),
                                "C" => Value::Char(i),
                                "I" => Value::Int(i),
                                "S" => Value::Short(i),
                                _ => return Err(Error::IllegalCastToInt),
                            },
                            Entry::Float(f) => Value::Float(f),
                            Entry::Long(l) => Value::Long(l),
                            Entry::Double(d) => Value::Double(d),
                            Entry::String(_s_index) => {
                                // This should make a new instance of String, but for now we will just give a null ref.
                                Value::Reference(Reference::Null)
                            },
                            _ => return Err(Error::IllegalConstantLoad(Opcode::ClassLoad)),
                        }
                    }
                    else {
                        match &name_and_type.descriptor[0..1] {
                            "B" | "Z" => Value::Byte(0),
                            "C" => Value::Char(0),
                            "I" => Value::Int(0),
                            "S" => Value::Short(0),
                            "F" => Value::Float(0.0),
                            "J" => Value::Long(0),
                            "D" => Value::Double(0.0),
                            "[" | "L" => {
                                Value::Reference(Reference::Null)
                            },
                            _ => return Err(Error::IllegalDescriptor),
                        }
                    }
                };
                static_fields.insert(name_and_type, Rc::new(value));
            }
        }
        #[cfg(target_family = "wasm")]
        let class = {
            CustomClass { class_file: Rc::new(file), static_fields}
        };
        #[cfg(not(target_family = "wasm"))]
        let mut class = {
            let module = jvm.context.create_module(file.name());
            let builder = jvm.context.create_builder();
            let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
    
            CustomClass { class_file: Rc::new(file), static_fields, context: jvm.context, module, builder, execution_engine}
        };

        #[cfg(not(target_family = "wasm"))]
        if jvm.should_always_jit {
            for method in &class.get_class_file().methods {
                let mut can_jit = true;

                for op in method.code().unwrap() {
                    can_jit &= op.can_jit();
                }

                if can_jit {
                    class.try_codegen_fn(jvm, method);
                }
            } 
        }

        Ok(class)
    }
    fn get_static(&self, name: &str, descriptor: &str, jvm: &mut JVM) -> Result<Value<dyn Class, dyn Object>, Error> {
        let name_and_type = NameAndType { name: String::from(name), descriptor: String::from(descriptor) };
        if let Some(v) = self.static_fields.get(&name_and_type) {
            Ok((**v).clone())
        }
        else {
            for interface_index in &self.class_file.interfaces {
                let interface_name_index = self.class_file.cp_entry(*interface_index)?.as_class()?;
                let interface_name = self.class_file.cp_entry(*interface_name_index)?.as_utf8()?;
                let mut interface;
                let mut current_interface = &jvm.resolve_class_reference(interface_name)?;
                // Now we loop over the superclasses of the interface.
                while current_interface.get_class_file().has_super() {
                    if let Ok(v) = current_interface.get_static(name, descriptor, jvm) {
                        return Ok(v);
                    }
                    interface = jvm.resolve_class_reference(current_interface.get_class_file().super_name().unwrap())?;
                    current_interface = &interface;
                }
            }
            if self.class_file.has_super() {
                jvm.resolve_class_reference(self.class_file.super_name().unwrap())?.get_static(name, descriptor, jvm)
            }
            else {
                Err(Error::NoSuchFieldError(Opcode::GETSTATIC))
            }
        }
        /* 
        let mut class;
        let mut current_class = &jvm.resolve_class_reference(self.class_file.name())?;
        while current_class.get_class_file().has_super() {
            // First, check the current class's fields.
            if let Some(v) = current_class.get_class_file().static_fields.get(&name_and_type) {
                return Ok(v.clone());
            }
            // Next, we have to check the interfaces

            for interface_index in &current_class.get_class_file().interfaces {
                let interface_name_index = current_class.get_class_file().cp_entry(*interface_index)?.as_class()?;
                let interface_name = current_class.get_class_file().cp_entry(*interface_name_index)?.as_utf8()?;
                let mut interface;
                let mut current_interface = &jvm.resolve_class_reference(interface_name)?;
                // Now we loop over the superclasses of the interface.
                while current_interface.get_class_file().has_super() {
                    if let Some(v) = current_interface.get_class_file().m_static_fields.get(&name_and_type) {
                        return Ok(v.clone());
                    }
                    interface = jvm.resolve_class_reference(current_interface.get_class_file().super_name().unwrap())?;
                    current_interface = &interface;
                }
            }
            // Finally, we propogate to the superclasses of the current class.
            class = jvm.resolve_class_reference(current_class.get_class_file().super_name().unwrap())?;
            current_class = &class;
        }
        
        */
    }
    //https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-6.html#jvms-6.5.putstatic
    fn put_static(&mut self, name: &str, descriptor: &str, value: Value<dyn Class, dyn Object>, jvm: &mut JVM) -> Result<(), Error> {
        let name_and_type = NameAndType { name: String::from(name), descriptor: String::from(descriptor) };
        if let Some(v) = self.static_fields.get_mut(&name_and_type) {
            // FIXME: Check compatibility
            unsafe {
                *Rc::get_mut_unchecked(v) = value;
            }
            Ok(())
        }
        else if !self.class_file.has_super() {
            Err(Error::NoSuchFieldError(Opcode::PUTSTATIC))
        }
        else {
            unsafe {
                Rc::get_mut_unchecked(&mut jvm.resolve_class_reference(self.class_file.super_name().unwrap())?)
            }.put_static(name, descriptor, value, jvm)
        }
        
    }
    fn exec_method(self: Rc<Self>, jvm: &mut JVM, method: &MethodInfo) -> Result<bool, Error> {
        let c_file = self.get_class_file();

        if method.access_flags & flags::method::ACC_NATIVE {
            eprintln!("{}", 
            format!("WARNING: Native method {}.{}{} was called, skipping", 
                c_file.name(),
                c_file.cp_entry(method.name_index)?.as_utf8()?, 
                c_file.cp_entry(method.descriptor_index)?.as_utf8()?
            ).as_str().red());
            // TODO: Fake return value
            return Ok(false);
        }

        let thread = access_macros::current_thread_mut!(jvm);
        // Fill out the local variables.
        
        // Use jvm::parse_descriptor
        let (local_types, _) = JVM::parse_descriptor(c_file.cp_entry(method.descriptor_index)?.as_utf8()?)?;
        let mut new_frame = Frame::new(self.clone(), method.clone(), 
            method.code.as_ref().unwrap().max_locals.into());
        let locals = &mut new_frame.local_variables;
        
        if local_types.len() > 0 {
            let mut val_idx: usize = local_types.len();

            while val_idx > 0 {
                let val = local_types[val_idx - 1];
                match val {
                    ValueMarker::Byte => {
                        let current_frame = access_macros::current_frame_mut!(thread);
                        let val = match current_frame.op_stack.pop() {
                            Some(v) => v,
                            None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                        };
                        let inner_value = Value::to_int(val)?;
                        locals[val_idx - 1] = VarValue::Byte(inner_value);
                    },
                    ValueMarker::Char => {
                        let current_frame = access_macros::current_frame_mut!(thread);
                        let val = match current_frame.op_stack.pop() {
                            Some(v) => v,
                            None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                        };
                        let inner_value = Value::to_int(val)?;
                        locals[val_idx - 1] = VarValue::Char(inner_value);
                    },
                    ValueMarker::Double => {
                        let current_frame = access_macros::current_frame_mut!(thread);
                        locals[val_idx - 1] = VarValue::DoubleHighBytes;
                        val_idx -= 1;
                        let val = match current_frame.op_stack.pop() {
                            Some(v) => v,
                            None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                        };
                        let inner_value = Value::to_double(val)?;
                        locals[val_idx - 1] = VarValue::Double(inner_value);
                    },
                    ValueMarker::Float => {
                        let current_frame = access_macros::current_frame_mut!(thread);
                        let val = match current_frame.op_stack.pop() {
                            Some(v) => v,
                            None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                        };
                        let inner_value = Value::to_float(val)?;
                        locals[val_idx - 1] = VarValue::Float(inner_value);
                    },
                    ValueMarker::Int => {
                        let current_frame = access_macros::current_frame_mut!(thread);
                        let val = match current_frame.op_stack.pop() {
                            Some(v) => v,
                            None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                        };
                        let inner_value = Value::to_int(val)?;
                        locals[val_idx - 1] = VarValue::Int(inner_value);
                        
                    },
                    ValueMarker::Long => {
                        let current_frame = access_macros::current_frame_mut!(thread);
                        locals[val_idx - 1] = VarValue::LongHighBytes;
                        val_idx -=  1;
                        let val = match current_frame.op_stack.pop() {
                            Some(v) => v,
                            None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                        };
                        let inner_value = Value::to_long(val)?;
                        locals[val_idx - 1] = VarValue::Long(inner_value);
                    },
                    ValueMarker::Short => {
                        let current_frame = access_macros::current_frame_mut!(thread);
                        let val = match current_frame.op_stack.pop() {
                            Some(v) => v,
                            None => return Err(Error::StackUnderflow(Opcode::MethodInvoke)),
                        };
                        let inner_value = Value::to_int(val)?;
                        locals[val_idx - 1] = VarValue::Short(inner_value);
                    },
                    ValueMarker::Reference => {            
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
                            else if self.get_class_file().name() != jvm.m_main_class_name {
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
                        locals[val_idx - 1] = VarValue::Reference(inner_value);
                    }
                    ValueMarker::Void => {
                        panic!("There shouldn't be void types in arguments");
                    }
                    ValueMarker::Top => {
                        panic!("We shouldn't read top type");
                    },
                }
                val_idx -= 1;
            }  
        }
        /* 
        print!("Local types for static execution: [");
        for ty in &*local_types {
            print!(" {ty:?}, ");
        }
        println!("]");

        print!("Current local vars: [");
        for var in locals {
            print!(" {var}, ");
        }
        println!("]");
        */

        thread.push_frame(new_frame);
        Ok(false)
    }
    fn get_class_file(&self) -> Rc<ClassFile> {
        self.class_file.clone()
    }
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn Class> {
        self
    }
    fn as_any(&self) ->  &dyn Any {
        self
    }
    fn as_any_rc(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

#[cfg(not(target_family = "wasm"))]
use {
    inkwell::OptimizationLevel,
    inkwell::basic_block::BasicBlock,
    inkwell::builder::Builder,
    inkwell::context::Context,
    inkwell::execution_engine::{ExecutionEngine, JitFunction},
    inkwell::module::Module,
    inkwell::types::{BasicType, BasicMetadataTypeEnum},
    inkwell::targets::{InitializationConfig, Target},
    inkwell::values::{IntValue, FunctionValue},
};

impl CustomClass {
    pub fn gen_block_indexes(&self, method: &MethodInfo) -> Vec<usize> {
        // Create block layout
        todo!()
    }
    #[cfg(not(target_family = "wasm"))]
    fn try_codegen_fn(&mut self, jvm: &mut JVM, method: &MethodInfo) {
        // Maybe put this verification pass somewhere else
        let mut can_jit = true;
        for op in method.code().unwrap() {
            can_jit &= op.can_jit();
        }
        if !can_jit { return; }

        let c_file = self.get_class_file();
        let mut fname = String::from(c_file.cp_entry(method.name_index).unwrap().as_utf8().unwrap());
        fname.push_str(c_file.cp_entry(method.descriptor_index).unwrap().as_utf8().unwrap());
        let (args, ret) = JVM::parse_descriptor(c_file.cp_entry(method.descriptor_index).
            unwrap().as_utf8().unwrap()).unwrap();
        let fn_type = ret.llvm_type(self.context).fn_type(
            args.into_iter()
            .map(|t| t.llvm_type(self.context).into())
            .collect::<Vec<BasicMetadataTypeEnum>>().as_slice(), false);
        let function = self.module.add_function(fname.as_str(), fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
        // Create locals
        let local_var_layout = stack_map_table::local_var_layout(
            method.code.as_ref().unwrap().stack_map_table.as_ref().unwrap());
        if local_var_layout.is_none() { return; }
        let local_var_layout = local_var_layout.unwrap();

        let mut local_vars = Vec::new();
        for local_num in 0..local_var_layout.len() {
            if local_var_layout[local_num].is_none() { continue; }
            if local_var_layout[local_num].unwrap() == ValueMarker::Reference { return; } // Skip these for now 
            local_vars.push(self.builder.build_alloca(local_var_layout[local_num].unwrap().llvm_type(self.context), 
                format!("l{local_num}").as_str()));
        }

        let block_indexes = self.gen_block_indexes(method);
        let mut blocks = HashMap::new();
        for index in block_indexes {
            blocks.insert(index, self.context.append_basic_block(function, format!("block{index}").as_str()));
        }

        // Create op stack
        let stack = self.builder.build_array_alloca(self.context.i128_type(), 
            self.context.i32_type().const_int(method.code.as_ref().unwrap().max_stack.into(), false), "op_stack");
        let stack_top = self.builder.build_alloca(self.context.i64_type(), "stack_top");

        
        let mut op_idx = 0;
        for op in method.code().unwrap() {
            op_idx += 1;
            if let Some(block) = blocks.get(&op_idx) { self.builder.position_at_end(*block) }

            op.jit(self.context, &self.module, &self.builder, &self.execution_engine, 
                &fname, function, &local_vars, &blocks, &stack, &stack_top);
        }
        
    }
}


