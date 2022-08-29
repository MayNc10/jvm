// These types store a representaytion of a JVM Value.
// Value is the most basic, just using a union to hold every type.
// It is used for storing values of fields, and on the operand stack.
// VarValue is like this, except it uses an emun that can represent the high bytes of longs and doubles, 
// in accordance with the spec for the local variable table

// Idea: Consider using alloc::borrow::Cow to keep stack values.
// https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html
use std::result::Result;
use std::rc::Rc;

use crate::errorcodes::Error;
use crate::reference;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Byte(i32),
    Short(i32),
    Int(i32),
    Long(i64),
    Char(i32),
    Float(f32),
    Double(f64),
    ReturnAddress(u16),
    Reference(Rc<reference::Reference>)
}

impl Value {
    pub fn as_int(&self) -> Result<&i32, Error> {
        match self {
            Value::Byte(i) | Value::Short(i) | Value::Int(i) | Value::Char(i) => Ok(i),
           _ => Err(Error::IllegalCastToInt),
        }
    }
    pub fn as_long(&self) -> Result<&i64, Error> {
        if let Value::Long(l) = self {
            return Ok(l);
        }
        Err(Error::IllegalCastToLong)
    }
    pub fn as_float(&self) -> Result<&f32, Error> {
        if let Value::Float(f) = self {
            return Ok(f);
        }
        Err(Error::IllegalCastToFloat)
    }
    pub fn as_double(&self) -> Result<&f64, Error> {
        if let Value::Double(d) = self {
            return Ok(d);
        }
        Err(Error::IllegalCastToDouble)
    }
    pub fn as_retaddr(&self) -> Result<&u16, Error> {
        if let Value::ReturnAddress(addr) = self {
            return Ok(addr);
        }
        Err(Error::IllegalCastToReturnAddress)
    }
    pub fn as_reference(&self) -> Result<Rc<reference:: Reference>, Error> {
        if let Value::Reference(reference) = self {
            return Ok(reference.clone());
        }
        Err(Error::IllegalCastToReference)
    }

    pub fn as_int_mut(&mut self) -> Result<&mut i32, Error> {
        match self {
            Value::Byte(i) | Value::Short(i) | Value::Int(i) | Value::Char(i) => Ok(i),
           _ => Err(Error::IllegalCastToInt),
        }
    }
    pub fn as_long_mut(&mut self) -> Result<&mut i64, Error> {
        if let Value::Long(l) = self {
            return Ok(l);
        }
        Err(Error::IllegalCastToLong)
    }
    pub fn as_float_mut(&mut self) -> Result<&mut f32, Error> {
        if let Value::Float(f) = self {
            return Ok(f);
        }
        Err(Error::IllegalCastToFloat)
    }
    pub fn as_double_mut(&mut self) -> Result<&mut f64, Error> {
        if let Value::Double(d) = self {
            return Ok(d);
        }
        Err(Error::IllegalCastToDouble)
    }
    pub fn as_retaddr_mut(&mut self) -> Result<&mut u16, Error> {
        if let Value::ReturnAddress(addr) = self {
            return Ok(addr);
        }
        Err(Error::IllegalCastToReturnAddress)
    }
    pub fn as_reference_mut(&mut self) -> Result<Rc<reference:: Reference>, Error> {
        if let Value::Reference(reference) = self {
            return Ok(reference.clone());
        }
        Err(Error::IllegalCastToReference)
    }

    pub fn new(descriptor: &str) -> Value {
        // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-4.html#jvms-4.2.2
        // Because we aren't parsing a full type, we can just look at the first character

        // These 'as_chars' calls are valid, because if we get stuck inside a character, we still should fail for the same reason as normal.
        // These are all just numbers, because Rust doesn't allow comparisons between u8s and chars.
        match descriptor.as_bytes()[0] {
            66 | 90 => Value::Byte(0), // Bools are accessed through the byte instructions, so it makes sense to create one as a byte.
            67 => Value::Char(0),
            68 => Value::Double(0.0),
            70 => Value::Float(0.0),
            73 => Value::Int(0),
            74 => Value::Long(0),
            83 => Value::Short(0),
            76 | 91 => Value::Reference(Rc::new(reference::Reference::Null)), // We don't care about the full type, just that it's a reference. 
            _ => panic!("Type descriptor contains illegal first character: {}", descriptor.as_bytes()[0]),
        }
    }

}

impl Value {
    pub fn is_int(&self) -> bool {
        match self {
            Value::Byte(_) | Value::Short(_) | Value::Int(_) | Value::Char(_) => true,
            _ => false, 
        }
    }
    pub fn is_long(&self) -> bool {
        match self {
            Value::Long(_) => true,
            _ => false,
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            Value::Float(_) => true,
            _ => false,
        }
    }
    pub fn is_double(&self) -> bool {
        match self {
            Value::Double(_) => true,
            _ => false,
        }
    }
    pub fn is_retaddr(&self) -> bool {
        match self {
            Value::ReturnAddress(_) => true,
            _ => false,
        }
    }
    pub fn is_reference(&self) -> bool {
        match self {
            Value::Reference(_) => true,
            _ => false,
        }
    }
    pub fn to_int(val: Value) -> Result<i32, Error> {
        match val {
            Value::Byte(i) | Value::Short(i) | Value::Int(i) | Value::Char(i) => Ok(i),
           _ => Err(Error::IllegalCastToInt),
        }
    }
    pub fn to_long(val: Value) -> Result<i64, Error> {
        if let Value::Long(l) = val {
            return Ok(l);
        }
        Err(Error::IllegalCastToLong)
    }
    pub fn to_float(val: Value) -> Result<f32, Error> {
        if let Value::Float(f) = val {
            return Ok(f);
        }
        Err(Error::IllegalCastToFloat)
    }
    pub fn to_double(val: Value) -> Result<f64, Error> {
        if let Value::Double(d) = val {
            return Ok(d);
        }
        Err(Error::IllegalCastToDouble)
    }
    pub fn to_retaddr(val: Value) -> Result<u16, Error> {
        if let Value::ReturnAddress(addr) = val {
            return Ok(addr);
        }
        Err(Error::IllegalCastToReturnAddress)
    }
    pub fn to_reference(val: Value) -> Result<Rc<reference::Reference>, Error> {
        if let Value::Reference(reference) = val {
            return Ok(reference);
        }
        Err(Error::IllegalCastToReference)
    }

    pub fn is_comptype1(&self) -> bool {
        self.is_long() || self.is_double()
    }
    pub fn is_comptype2(&self) -> bool {
        !self.is_comptype1()
    }
}

#[derive(Clone, Debug)]
pub enum VarValue {
    Byte(i32),
    Short(i32),
    Int(i32),
    Long(i64),
    LongHighBytes,
    Char(i32),
    Float(f32),
    Double(f64),
    DoubleHighBytes,
    ReturnAddress(u16),
    Reference(Rc<reference:: Reference>)
}

impl<'a> VarValue {
    pub fn as_int(&self) -> Result<&i32, Error> {
        match self {
            VarValue::Byte(i) | VarValue::Short(i) | VarValue::Int(i) | VarValue::Char(i) => Ok(i),
           _ => Err(Error::IllegalCastToInt),
        }
    }
    pub fn as_long(&self) -> Result<&i64, Error> {
        if let VarValue::Long(l) = self {
            return Ok(l);
        }
        Err(Error::IllegalCastToLong)
    }
    pub fn as_float(&self) -> Result<&f32, Error> {
        if let VarValue::Float(f) = self {
            return Ok(f);
        }
        Err(Error::IllegalCastToFloat)
    }
    pub fn as_double(&self) -> Result<&f64, Error> {
        if let VarValue::Double(d) = self {
            return Ok(d);
        }
        Err(Error::IllegalCastToDouble)
    }
    pub fn as_retaddr(&self) -> Result<&u16, Error> {
        if let VarValue::ReturnAddress(addr) = self {
            return Ok(addr);
        }
        Err(Error::IllegalCastToReturnAddress)
    }
    pub fn as_reference(&self) -> Result<Rc<reference:: Reference>, Error> {
        if let VarValue::Reference(reference) = self {
            return Ok(reference.clone());
        }
        Err(Error::IllegalCastToReference)
    }

    pub fn as_int_mut(&mut self) -> Result<&mut i32, Error> {
        match self {
            VarValue::Byte(i) | VarValue::Short(i) | VarValue::Int(i) | VarValue::Char(i) => Ok(i),
           _ => Err(Error::IllegalCastToInt),
        }
    }
    pub fn as_long_mut(&mut self) -> Result<&mut i64, Error> {
        if let VarValue::Long(l) = self {
            return Ok(l);
        }
        Err(Error::IllegalCastToLong)
    }
    pub fn as_float_mut(&mut self) -> Result<&mut f32, Error> {
        if let VarValue::Float(f) = self {
            return Ok(f);
        }
        Err(Error::IllegalCastToFloat)
    }
    pub fn as_double_mut(&mut self) -> Result<&mut f64, Error> {
        if let VarValue::Double(d) = self {
            return Ok(d);
        }
        Err(Error::IllegalCastToDouble)
    }
    pub fn as_retaddr_mut(&mut self) -> Result<&mut u16, Error> {
        if let VarValue::ReturnAddress(addr) = self {
            return Ok(addr);
        }
        Err(Error::IllegalCastToReturnAddress)
    }
    pub fn as_reference_mut(&mut self) -> Result<Rc<reference:: Reference>, Error> {
        if let VarValue::Reference(reference) = self {
            return Ok(reference.clone());
        }
        Err(Error::IllegalCastToReference)
    }

    pub fn new(descriptor: &str) -> VarValue {
        // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-4.html#jvms-4.2.2
        // Because we aren't parsing a full type, we can just look at the first character

        // These 'as_bytes' calls are valid, because if we get stuck inside a character, we still should fail for the same reason as normal.
        // These are all just numbers, because Rust doesn't allow comparisons between u8s and chars.
        match descriptor.as_bytes()[0] {
            66 | 90 => VarValue::Byte(0), // Bools are accessed through the byte instructions, so it makes sense to create one as a byte.
            67 => VarValue::Char(0),
            68 => VarValue::Double(0.0),
            70 => VarValue::Float(0.0),
            73 => VarValue::Int(0),
            74 => VarValue::Long(0),
            83 => VarValue::Short(0),
            76 | 91 => VarValue::Reference(Rc::new(reference::Reference::Null)), // We don't care about the full type, just that it's a reference. 
            _ => panic!("Type descriptor contains illegal first character: {}", descriptor.as_bytes()[0]),
        }
    }

}



