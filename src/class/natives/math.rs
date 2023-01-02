use std::rc::Rc;
use colored::Colorize;
use rand::prelude::*;

use super::super::*;
use crate::{errorcodes::Opcode, reference::{Reference, object::{self, natives}, Monitor, array::{Array, RefArray}}, frame::Frame};

pub struct Math {
    file: Rc<ClassFile>
}

impl Class for Math {
    fn new(file: ClassFile, _jvm: &mut JVM) -> Result<Self, Error> where Self : Sized {
        Ok( Math {
            file: Rc::new(file),
        })
    }
    fn get_static(&self, name: &str, _descriptor: &str, _jvm: &mut JVM) -> Result<Value<dyn Class, dyn Object>, Error> {
        match name {
            "E" => Ok(Value::Double(std::f64::consts::E)),
            "PI" => Ok(Value::Double(std::f64::consts::PI)),
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
            ("abs", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.abs()));
            },
            ("abs", "(F)F") => {
                let val = frame.op_stack.pop().unwrap().to_float()?;
                frame.op_stack.push(Value::Float(val.abs()));
            },
            ("abs", "(I)I") => {
                let val = frame.op_stack.pop().unwrap().to_int()?;
                frame.op_stack.push(Value::Int(val.abs()));
            },
            ("abs", "(J)J") => {
                let val = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long(val.abs()));
            },
            ("absExact", "(I)I") => {
                let val = frame.op_stack.pop().unwrap().to_int()?;
                frame.op_stack.push(Value::Int(val.abs()));
            },
            ("absExact", "(J)J") => {
                let val = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long(val.abs()));
            },
            ("acos", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.acos()));
            },
            ("addExact", "(I)I") => {
                let val = frame.op_stack.pop().unwrap().to_int()?;
                frame.op_stack.push(Value::Int(val.abs()));
            },
            ("addExact", "(J)J") => {
                let val = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long(val.abs()));
            },
            ("asin", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.asin()));
            },
            ("atan", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.atan()));
            },
            ("atan2", "(DD)D") => {
                let x = frame.op_stack.pop().unwrap().to_double()?;
                let y = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(y.atan2(x)));
            },
            ("cbrt", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.cbrt()));
            },
            ("ceil", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.ceil()));
            },
            ("copySign", "(DD)D") => {
                let sign = frame.op_stack.pop().unwrap().to_double()?;
                let mag = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(mag.copysign(sign)));
            },
            ("copySign", "(FF)F") => {
                let sign = frame.op_stack.pop().unwrap().to_float()?;
                let mag = frame.op_stack.pop().unwrap().to_float()?;
                frame.op_stack.push(Value::Float(mag.copysign(sign)));
            },
            ("cos", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.cos()));
            },
            ("cosh", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.cosh()));
            },
            ("decrementExact", "(I)I") => {
                let val = frame.op_stack.pop().unwrap().to_int()?;
                frame.op_stack.push(Value::Int(val - 1));
            },
            ("decrementExact", "(J)J") => {
                let val = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long(val - 1));
            },
            ("exp", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.exp()));
            },
            ("expm1", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.exp_m1()));
            },
            ("floor", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.floor()));
            },
            ("floorDiv", "(II)I") => {
                let y = frame.op_stack.pop().unwrap().to_int()?;
                let x = frame.op_stack.pop().unwrap().to_int()?;
                frame.op_stack.push(Value::Int(x.div_floor(y)));
            },
            ("floorDiv", "(JI)J") => {
                let y = frame.op_stack.pop().unwrap().to_int()? as i64;
                let x = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long(x.div_floor(y)));
            },
            ("floorDiv", "(JJ)J") => {
                let y = frame.op_stack.pop().unwrap().to_long()?;
                let x = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long(x.div_floor(y)));
            },
            ("floorMod", "(II)I") => {
                let y = frame.op_stack.pop().unwrap().to_int()?;
                let x = frame.op_stack.pop().unwrap().to_int()?;
                frame.op_stack.push(Value::Int((x % y) + if y.signum() != x.signum() {y} else { 0 }));
            },
            ("floorMod", "(JI)J") => {
                let y = frame.op_stack.pop().unwrap().to_int()? as i64;
                let x = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long((x % y) + if y.signum() != x.signum() {y} else { 0 }));
            },
            ("floorMod", "(JJ)J") => {
                let y = frame.op_stack.pop().unwrap().to_long()?;
                let x = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long((x % y) + if y.signum() != x.signum() {y} else { 0 }));
            },
            ("fma", "(DDD)D") => {
                let c = frame.op_stack.pop().unwrap().to_double()?;
                let b = frame.op_stack.pop().unwrap().to_double()?;
                let a = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(a.mul_add(b, c)));
            },
            ("fma", "(FFF)F") => {
                let c = frame.op_stack.pop().unwrap().to_float()?;
                let b = frame.op_stack.pop().unwrap().to_float()?;
                let a = frame.op_stack.pop().unwrap().to_float()?;
                frame.op_stack.push(Value::Float(a.mul_add(b, c)));
            },
            ("getExponent", "(D)I") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Int((val.abs().to_bits() >> f64::MANTISSA_DIGITS) as i32));
            },
            ("getExponent", "(F)I") => {
                let val = frame.op_stack.pop().unwrap().to_float()?;
                frame.op_stack.push(Value::Int((val.abs().to_bits() >> f32::MANTISSA_DIGITS) as i32));
            },
            ("hypot", "(DD)D") => {
                let y = frame.op_stack.pop().unwrap().to_double()?;
                let x = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(x.hypot(y)));
            },
            ("IEEEremainder", "(DD)D") => {
                let f2 = frame.op_stack.pop().unwrap().to_double()?;
                let f1 = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(f1 % f2));
            },
            ("incrementExact", "(I)I") => {
                let val = frame.op_stack.pop().unwrap().to_int()?;
                frame.op_stack.push(Value::Int(val + 1));
            },
            ("incrementExact", "(J)J") => {
                let val = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long(val + 1));
            },
            ("log", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.ln()));
            },
            ("log10", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.log10()));
            },
            ("log1p", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.ln_1p()));
            },
            ("max", "(DD)D") => {
                let y = frame.op_stack.pop().unwrap().to_double()?;
                let x = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(x.max(y)));
            },
            ("max", "(FF)F") => {
                let y = frame.op_stack.pop().unwrap().to_float()?;
                let x = frame.op_stack.pop().unwrap().to_float()?;
                frame.op_stack.push(Value::Float(x.max(y)));
            },
            ("max", "(II)I") => {
                let y = frame.op_stack.pop().unwrap().to_int()?;
                let x = frame.op_stack.pop().unwrap().to_int()?;
                frame.op_stack.push(Value::Int(x.max(y)));
            },
            ("max", "(JJ)J") => {
                let y = frame.op_stack.pop().unwrap().to_long()?;
                let x = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long(x.max(y)));
            },
            ("min", "(DD)D") => {
                let y = frame.op_stack.pop().unwrap().to_double()?;
                let x = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(x.min(y)));
            },
            ("min", "(FF)F") => {
                let y = frame.op_stack.pop().unwrap().to_float()?;
                let x = frame.op_stack.pop().unwrap().to_float()?;
                frame.op_stack.push(Value::Float(x.min(y)));
            },
            ("min", "(II)I") => {
                let y = frame.op_stack.pop().unwrap().to_int()?;
                let x = frame.op_stack.pop().unwrap().to_int()?;
                frame.op_stack.push(Value::Int(x.min(y)));
            },
            ("min", "(JJ)J") => {
                let y = frame.op_stack.pop().unwrap().to_long()?;
                let x = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long(x.min(y)));
            },
            ("multiplyExact", "(II)I") => {
                let y = frame.op_stack.pop().unwrap().to_int()?;
                let x = frame.op_stack.pop().unwrap().to_int()?;
                frame.op_stack.push(Value::Int(x * y));
            },
            ("multiplyExact", "(JI)J") => {
                let y = frame.op_stack.pop().unwrap().to_int()? as i64;
                let x = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long(x * y));
            },
            ("multiplyExact", "(JJ)J") => {
                let y = frame.op_stack.pop().unwrap().to_long()?;
                let x = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long(x * y));
            },
            ("multiplyFull", "(II)J") => {
                let y = frame.op_stack.pop().unwrap().to_int()? as i64;
                let x = frame.op_stack.pop().unwrap().to_int()? as i64;
                frame.op_stack.push(Value::Long(x * y));
            },
            ("multiplyHigh", "(JJ)J") => {
                let y = frame.op_stack.pop().unwrap().to_long()? as i128;
                let x = frame.op_stack.pop().unwrap().to_long()? as i128;
                frame.op_stack.push(Value::Long(((x * y) >> 64) as i64));
            },
            ("negateExact", "(I)I") => {
                let val = frame.op_stack.pop().unwrap().to_int()?;
                frame.op_stack.push(Value::Int(val * -1));
            },
            ("negateExact", "(J)J") => {
                let val = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long(val * -1));
            },
            ("nextAfter", "(DD)D") => {
                let y = frame.op_stack.pop().unwrap().to_double()?;
                let x = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(if y < x {x.next_down()} else {x.next_up()}));
            },
            ("nextAfter", "(FF)F") => {
                let y = frame.op_stack.pop().unwrap().to_float()?;
                let x = frame.op_stack.pop().unwrap().to_float()?;
                frame.op_stack.push(Value::Float(if y < x {x.next_down()} else {x.next_up()}));
            },
            ("nextDown", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.next_down()));
            },
            ("nextDown", "(F)F") => {
                let val = frame.op_stack.pop().unwrap().to_float()?;
                frame.op_stack.push(Value::Float(val.next_down()));
            },
            ("nextUp", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.next_up()));
            },
            ("nextUp", "(F)F") => {
                let val = frame.op_stack.pop().unwrap().to_float()?;
                frame.op_stack.push(Value::Float(val.next_up()));
            },
            ("pow", "(DD)D") => {
                let y = frame.op_stack.pop().unwrap().to_double()?;
                let x = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(x.powf(y)));
            },
            ("random", "()D") => {
                frame.op_stack.push(Value::Double(random()));
            },
            ("rint", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.round()));
            },
            ("round", "(D)J") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Long(val.round() as i64)); // doesn't do rounding right but whatever
            },
            ("round", "(F)I") => {
                let val = frame.op_stack.pop().unwrap().to_float()?;
                frame.op_stack.push(Value::Int(val.round() as i32)); // doesn't do rounding right but whatever
            },
            ("scalb", "(DI)D") => {
                let factor = frame.op_stack.pop().unwrap().to_int()?;
                let d = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(d * 2.0_f64.powi(factor))); // Just wrong 
            },
            ("scalb", "(FI)F") => {
                let factor = frame.op_stack.pop().unwrap().to_int()?;
                let f = frame.op_stack.pop().unwrap().to_float()?;
                frame.op_stack.push(Value::Float(f * 2.0_f32.powi(factor))); // Just wrong 
            },
            ("signum", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.signum()));
            },
            ("signum", "(F)F") => {
                let val = frame.op_stack.pop().unwrap().to_float()?;
                frame.op_stack.push(Value::Float(val.signum()));
            },
            ("sin", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.sin()));
            },
            ("sinh", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.sinh()));
            },
            ("sqrt", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.sqrt()));
            },
            ("subtractExact", "(II)I") => {
                let y = frame.op_stack.pop().unwrap().to_int()?;
                let x = frame.op_stack.pop().unwrap().to_int()?;
                frame.op_stack.push(Value::Int(x - y));
            },
            ("subtractExact", "(JJ)J") => {
                let y = frame.op_stack.pop().unwrap().to_long()?;
                let x = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Long(x - y));
            },
            ("tan", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.tan()));
            },
            ("tanh", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.tanh()));
            },
            ("toDegrees", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.to_degrees()));
            },
            ("toIntExact", "(J)I") => {
                let val = frame.op_stack.pop().unwrap().to_long()?;
                frame.op_stack.push(Value::Int(val as i32));
            },
            ("toRadians", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(val.to_radians()));
            },
            ("ulp", "(D)D") => {
                let val = frame.op_stack.pop().unwrap().to_double()?;
                frame.op_stack.push(Value::Double(if val.is_sign_positive() {
                    val.next_up() - val
                }
                else {
                    val - val.next_down()
                }));
            },
            ("ulp", "(F)F") => {
                let val = frame.op_stack.pop().unwrap().to_float()?;
                frame.op_stack.push(Value::Float(if val.is_sign_positive() {
                    val.next_up() - val
                }
                else {
                    val - val.next_down()
                }));
            },
            _ => {
                // do funky stuff
                eprintln!("{}", format!("Use of unimplemented function: {name}{desc} in class Math").red());
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
