use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicMetadataTypeEnum};
use inkwell::targets::{InitializationConfig, Target};

use std::{collections::HashMap, rc::Rc};

use crate::{constant_pool::{NameAndType, Entry}, value::{Value, VarValue}, errorcodes::Opcode, flags, reference::{Reference, array::Array, Monitor}, access_macros, frame::Frame};

use super::*;

pub struct CustomClass {
    class_file: Rc<classfile::ClassFile>,
    static_fields: HashMap<NameAndType, Rc<Value<dyn Class, dyn Object>>>, 
    context: &'static Context,
    module: Module<'static>,
    builder: Builder<'static>,
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
        let module = jvm.context.create_module(file.name());
        let builder = jvm.context.create_builder();
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

        let class = CustomClass { class_file: Rc::new(file), static_fields, context: jvm.context, module, builder, execution_engine};

        if jvm.should_always_jit {
            for method in &class.get_class_file().methods {
                let mut can_jit = true;

                for op in method.code().unwrap() {
                    can_jit &= op.can_jit();
                }

                if can_jit {
                    //class.codegen_fn(jvm, method);
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
        let thread = access_macros::current_thread_mut!(jvm);
        let mut new_frame = Frame::new(self.clone(), method.clone());
        // Fill out the local variables.
        let c_file = self.get_class_file();
        // Use jvm::parse_descriptor
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
                    locals.push(VarValue::Reference(inner_value));
                    descriptor = &descriptor[..index];
                }
                _ => return Err(Error::IllegalDescriptor),
            }
        }
        locals.reverse(); // We have to push the variables in reverse order, so we correct it after.   
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

/* 
impl CustomClass {
    fn codegen_fn(&mut self, jvm: &mut JVM, method: &MethodInfo) {
        // Maybe put this verification pass somewhere else
        let mut can_jit = true;
        for op in method.code().unwrap() {
            can_jit &= op.can_jit();
        }
        assert!(can_jit);

        let c_file = self.get_class_file();
        let mut fname = String::from(c_file.cp_entry(method.name_index).unwrap().as_utf8().unwrap());
        fname.push_str(c_file.cp_entry(method.descriptor_index).unwrap().as_utf8().unwrap());
        let (args, ret) = JVM::parse_descriptor(c_file.cp_entry(method.descriptor_index).
            unwrap().as_utf8().unwrap()).unwrap();
        let fn_type = ret.llvm_type(self.context).fn_type(
            args.into_iter().map(|t| t.llvm_type(self.context).into()).collect::<Vec<BasicMetadataTypeEnum>>().as_slice(), false);
        let function = self.module.add_function(fname.as_str(), fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
        // Create locals
        for local_num in 0..method.code.unwrap().max_locals {
            let desc = method.code.unwrap().stack_map_table.unwrap().
            self.builder.build_alloca(, name)
        }

        for op in method.code().unwrap() {
            op.jit(self.context, &self.module, &self.builder, &self.execution_engine, &fname, function);
        }
    }
}
*/

