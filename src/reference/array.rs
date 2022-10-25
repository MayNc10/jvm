use std::any::Any;
use std::marker::PhantomData;
use std::result::Result;
use std::rc::Rc;

use super::object::Object;
use super::{Reference, Monitor};

// A packed boolean array is more memory efficient, but slightly slower. 
// Maybe we'll use them, maybe we won't. We'll see.
// use super::packedboolarray::PackedBoolArray;

use crate::class::Class;
use crate::errorcodes::{Error, Opcode};
use crate::value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefArray<C: Class + ?Sized, O: Object + ?Sized> {
    pub arr: Vec<Reference<C, O>>,
    // This descriptor could be massively improved by making it a recursive enum. For now, this works.
    pub descriptor: String, 
}

impl PartialEq for RefArray<dyn Class, dyn Object> {
    fn eq(&self, other: &Self) -> bool {
        self.arr == other.arr && self.descriptor == self.descriptor
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Array<C: Class + ?Sized, O: Object + ?Sized> {
    Bool(Vec<bool>),
    Char(Vec<u16>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    Byte(Vec<i8>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Ref(RefArray<C, O>),
}

// This could be an enum, but this also works (and is, I think, simpler)
pub mod atype {
    pub const T_BOOLEAN: u8 = 4;
    pub const T_CHAR: u8 = 5;
    pub const T_FLOAT: u8 = 6;
    pub const T_DOUBLE: u8 = 7;
    pub const T_BYTE: u8 = 8;
    pub const T_SHORT: u8 = 9;
    pub const T_INT: u8 = 10;
    pub const T_LONG: u8 = 11;
    // Special, not included in the spec, but used in this implementation
    pub const T_REF: u8 = 3;
}

// Currently we store a reference directly, instead of using an Rc(). 
// I think this makes sense, but I'm not 100% confident that that is the way it's supposed to be.
impl<C: Class + ?Sized, O: Object + ?Sized> Array<C, O> {
    pub fn new(size: usize, atype: u8) -> Array<C, O> {
        match atype {
            atype::T_BOOLEAN => Array::new_bool(size),
            atype::T_CHAR => Array::new_char(size),
            atype::T_FLOAT => Array::new_float(size),
            atype::T_DOUBLE => Array::new_double(size),
            atype::T_BYTE => Array::new_byte(size),
            atype::T_SHORT => Array::new_short(size),
            atype::T_INT => Array::new_int(size),
            atype::T_LONG => Array::new_long(size),
            _ => panic!("Illegal value for atype in newarray: {}", atype),
        }
    }

    // In order to make these type-safe, all the vectors must be initialized.

    pub fn new_bool(size: usize) -> Array<C, O> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(false);
        }
        Array::Bool(v)
    }
    pub fn new_char(size: usize) -> Array<C, O> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(0);
        }
        Array::Char(v)
    }
    pub fn new_float(size: usize) -> Array<C, O> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(0.0);
        }
        Array::Float(v)
    }
    pub fn new_double(size: usize) -> Array<C, O> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(0.0);
        }
        Array::Double(v)
    }
    pub fn new_byte(size: usize) -> Array<C, O> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(0);
        }
        Array::Byte(v)
    }
    pub fn new_short(size: usize) -> Array<C, O> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(0);
        }
        Array::Short(v)
    }
    pub fn new_int(size: usize) -> Array<C, O> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(0);
        }
        Array::Int(v)
    }
    pub fn new_long(size: usize) -> Array<C, O> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(0);
        }
        Array::Long(v)
    }
    pub fn new_ref(size: usize, descriptor: String) -> Array<C, O> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(Reference::Null);
        }
        Array::Ref(RefArray {
            arr: v,
            descriptor,
        })
    }
    pub fn len(&self) -> usize {
        match self {
            Array::Bool(bvec) => bvec.len(),
            Array::Char(cvec) => cvec.len(),
            Array::Float(fvec) => fvec.len(),
            Array::Double(dvec) => dvec.len(),
            Array::Byte(bvec) => bvec.len(),
            Array::Short(svec) => svec.len(),
            Array::Int(ivec) => ivec.len(),
            Array::Long(lvec) => lvec.len(),
            Array::Ref(refarray) => refarray.arr.len(),
        }
    }

}


// Making dimension start as a usize makes the code more readable.
// In this case I think we can make CC a parameterized type, because all the Values should have the same type.
// This is because they all represent ints, so the only valid Class type for them to be is all java/lang/Integer.
fn fill_multi_level<C, O, CC, OO>(dimension: usize, dimension_cap: usize, counts: &[Value<CC, OO>], base_type: u8, descriptor: String) -> Result<Array<C, O> , Error> 
where   
    C: Class + ?Sized,
    O: Object + ?Sized,
    CC: Class + ?Sized,
    OO: Object + ?Sized,
{
    if (dimension + 1) == dimension_cap {
        if base_type == atype::T_REF {
            return Ok(Array::new_ref(*counts[dimension].as_int()? as usize, descriptor));
        }
        else {
            return Ok(Array::new(*counts[dimension].as_int()? as usize, base_type));
        }
    }

    let size = *counts[dimension].as_int()? as usize;
    let mut arr = Array::new_ref(size, descriptor.clone());
    for index in 0..size {
        let mut new_desciptor = descriptor.clone();
        new_desciptor.remove(0);
        let _ = arr.set(index, Value::Reference(Reference::Array(Rc::new(fill_multi_level(dimension + 1, dimension_cap, counts, base_type, new_desciptor)?), Rc::new(Monitor::new()))));
    }
    Ok(arr)
    
    
} 

impl<C: Class + ?Sized, O: Object + ?Sized> Array<C, O> {
    // same reason as fill_multi_level
    pub fn new_multi<CC, OO>(dimensions: u8, counts: &[Value<CC, OO>], descriptor: String) -> Result<Array<C, O>, Error> 
    where   
        CC: Class + ?Sized,
        OO: Object + ?Sized,
    {
        // First, verify descriptor makes sense.
        fn size_and_base(descriptor: &str, base_count: u8) -> (u8, char) {
            match descriptor.as_bytes()[0] as char {
                '[' => return size_and_base(&descriptor[1..], base_count + 1),
                _ => return (base_count, descriptor.as_bytes()[base_count as usize] as char ),
            }
        }
        let (dimensionality, base_type_char) = size_and_base(&descriptor, 0);
        if dimensionality < dimensions {
            return Err(Error::IncompatibleDimensionalityAndDescriptor);
        }
        let base_type = match base_type_char {
            'B' => atype::T_BYTE,
            'C' => atype::T_CHAR,
            'D' => atype::T_DOUBLE,
            'F' => atype::T_FLOAT,
            'I' => atype::T_INT,
            'J' => atype::T_LONG,
            'S' => atype::T_SHORT,
            'Z' => atype::T_BOOLEAN,
            'L' => atype::T_REF,
            _ => return Err(Error::IllegalDescriptor),
        };
        fill_multi_level(0, dimensions as usize, counts, base_type, descriptor)
    } 
}

impl<C: Class + ?Sized, O: Object + ?Sized> Array<C, O> {
    pub fn get(&self, index: usize) -> Value<C, O> {
        match self {
            // As per https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-2.html, bool is accessed using byte array instructions. 
            // Therefore, this case returns a Value byte.
            Array::Bool(bvec) => {
                Value::Byte(bvec[index] as i32)
            }
            Array::Byte(bvec) => {
                Value::Byte(bvec[index].into())
            }
            Array::Short(svec) => {
                Value::Short(svec[index].into())
            }
            Array::Int(ivec) => {
                Value::Int(ivec[index])
            }
            Array::Long(lvec) => {
                Value::Long(lvec[index])
            }
            Array::Char(cvec) => {
                Value::Char(cvec[index] as i32)
            }
            Array::Float(fvec) => {
                Value::Float(fvec[index])
            }
            Array::Double(dvec) => {
                Value::Double(dvec[index])
            }
            Array::Ref(refarray) => {
                Value::Reference(refarray.arr[index].clone())
            }

        }
    }
    // This might have to be a dyn class once we start checking assignment compatability (if we are supposed to).
    pub fn set(&mut self, index: usize, val: Value<C, O>) -> Result<(), Error> {
        match self {
            // As per https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-2.html, bool is accessed using byte array instructions. 
            // Therefore, this case returns a Value byte.
            Array::Bool(bvec) => {
                bvec[index] = *val.as_int()? == 0;
                Ok(())
            }
            Array::Byte(bvec) => {
                bvec[index] = *val.as_int()? as i8;
                Ok(())
            }
            Array::Short(svec) => {
                svec[index] = *val.as_int()? as i16;
                Ok(())
            }
            Array::Int(ivec) => {
                ivec[index] = *val.as_int()?;
                Ok(())
            }
            Array::Long(lvec) => {
                lvec[index] = *val.as_long()?;
                Ok(())
            }
            Array::Char(cvec) => {
                cvec[index] = *val.as_int()? as u16;
                Ok(())
            }
            Array::Float(fvec) => {
                fvec[index] = *val.as_float()?;
                Ok(())
            }
            Array::Double(dvec) => {
                dvec[index] = *val.as_double()?;
                Ok(())
            }
            Array::Ref(refarray) => {
                let mut reference = val.as_reference()?;
                refarray.arr[index] = reference.clone();
                Ok(())
            }

        }
    }
    pub fn is_boolarray(&self) -> bool {
        if let Array::Bool(_) = self {
            return true;
        }
        false
    }
    pub fn is_carray(&self) -> bool {
        if let Array::Char(_) = self {
            return true;
        }
        false
    }
    pub fn is_farray(&self) -> bool {
        if let Array::Float(_) = self {
            return true;
        }
        false
    }
    pub fn is_darray(&self) -> bool {
        if let Array::Double(_) = self {
            return true;
        }
        false
    }
    pub fn is_barray(&self) -> bool {
        if let Array::Byte(_) = self {
            return true;
        }
        false
    }
    pub fn is_sarray(&self) -> bool {
        if let Array::Short(_) = self {
            return true;
        }
        false
    }
    pub fn is_iarray(&self) -> bool {
        if let Array::Int(_) = self {
            return true;
        }
        false
    }
    pub fn is_larray(&self) -> bool {
        if let Array::Long(_) = self {
            return true;
        }
        false
    }
    pub fn is_refarray(&self) -> bool {
        if let Array::Ref(_) = self {
            return true;
        }
        false
    }
    pub fn descriptor(&self) -> &str {
        match self {
            Array::Bool(_) => "[Z",
            Array::Byte(_) => "[B",
            Array::Char(_) => "[C",
            Array::Double(_) => "[D",
            Array::Float(_) => "[F",
            Array::Int(_) => "[I",
            Array::Long(_) => "[J",
            Array::Short(_) => "[S",
            Array::Ref(r) => r.descriptor.as_str(),
        }
    }
}

macro_rules! partial_eq_array {
    ($T:ty, $name:ident) => {
        impl PartialEq<$T> for Array<dyn Class, dyn Object> {
            fn eq(&self, other: &$T) -> bool {
                match self {
                    Array::$name(v) => v == other,
                    _ => false,
                }
            }
            fn ne(&self, other: &$T) -> bool {
                !self.eq(other)
            }
        }
    };
}

partial_eq_array!(Vec<bool>, Bool);
partial_eq_array!(Vec<u16>, Char);
partial_eq_array!(Vec<f64>, Double);
partial_eq_array!(Vec<f32>, Float);
partial_eq_array!(Vec<i64>, Long);
partial_eq_array!(Vec<i8>, Byte);
partial_eq_array!(Vec<i16>, Short);
partial_eq_array!(Vec<i32>, Int);
partial_eq_array!(RefArray<dyn Class, dyn Object>, Ref);


impl PartialEq for Array<dyn Class, dyn Object> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Array::Bool(v) => other == v,
            Array::Char(v)=> other == v,
            Array::Double(v)=> other == v,
            Array::Float(v)=> other == v,
            Array::Long(v)=> other == v,
            Array::Byte(v) => other == v,
            Array::Short(v)=> other == v,
            Array::Int(v)=> other == v,
            Array::Ref(v)=> other == v,
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}



