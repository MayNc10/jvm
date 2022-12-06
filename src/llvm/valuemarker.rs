use crate::class::Class;
use crate::errorcodes::Error;
use crate::reference::Reference;
use crate::reference::object::Object;
use crate::value::Value;
use crate::multitypebox::MultiTypeBox;

use std::mem::size_of;


#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueMarker {
    Byte,
    Short,
    Int,
    Long,
    Char,
    Float,
    Double,
    Reference,
    Void,
    Top, // Top bytes of long or double (not really)
}

impl ValueMarker {
    pub fn from(val: &Value<dyn Class, dyn Object>) -> Result<ValueMarker, Error> {
        match val {
            Value::Byte(_) => Ok(Self::Byte),
            Value::Short(_) => Ok(Self::Short),
            Value::Int(_) => Ok(Self::Int),
            Value::Long(_) => Ok(Self::Long),
            Value::Char(_) => Ok(Self::Char),
            Value::Float(_) => Ok(Self::Float),
            Value::Double(_) => Ok(Self::Double),
            Value::Reference(_) => Ok(Self::Reference),
            Value::ReturnAddress(_) => Err(Error::IllegalDescriptor) // Make a new error
        }
    }
    pub fn size(&self) -> u8 {
        (match self {
            Self::Byte => size_of::<u8>(),
            Self::Short => size_of::<u16>(),
            Self::Int => size_of::<i32>(),
            Self::Long => size_of::<i64>(),
            Self::Char => size_of::<u16>(),
            Self::Float => size_of::<f32>(),
            Self::Double => size_of::<f64>(),
            Self::Reference => size_of::<Reference<dyn Class, dyn Object>>(),
            Self::Void => 0,
            Self::Top => 0,
        }) as u8
    }
   
    pub fn to_value(&self, b: &MultiTypeBox, idx: usize) -> Option<Value<dyn Class, dyn Object>> {
        Some( match self {
            ValueMarker::Byte => Value::Byte(*b.get::<i8>(idx)? as i32),
            ValueMarker::Short => Value::Short(*b.get::<i16>(idx)? as i32),
            ValueMarker::Int => Value::Int(*b.get(idx)?),
            ValueMarker::Long => Value::Long(*b.get(idx)?),
            ValueMarker::Char => Value::Char(*b.get::<u16>(idx)? as i32),
            ValueMarker::Float => Value::Float(*b.get(idx)?),
            ValueMarker::Double => Value::Double(*b.get(idx)?),
            ValueMarker::Reference => Value::Reference(b.get::<Reference<dyn Class, dyn Object>>(idx)?.clone()),
            ValueMarker::Void => return None,
            ValueMarker::Top => return None,
        } )
    }
}

impl PartialOrd for ValueMarker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size().partial_cmp(&other.size())
    }
}

impl Ord for ValueMarker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size().cmp(&other.size())
    }
}

#[cfg(not(target_family = "wasm"))]
use {
    inkwell::AddressSpace,
    inkwell::context::Context,
    inkwell::types::BasicTypeEnum,
};

impl ValueMarker {
    #[cfg(not(target_family = "wasm"))]
    pub fn llvm_type<'a>(&'a self, ctx: &'static Context) -> BasicTypeEnum<'static> {
        match self {
            Self::Byte => BasicTypeEnum::IntType(ctx.i8_type()),
            Self::Short => BasicTypeEnum::IntType(ctx.i16_type()),
            Self::Int => BasicTypeEnum::IntType(ctx.i32_type()),
            Self::Long => BasicTypeEnum::IntType(ctx.i64_type()),
            Self::Char => BasicTypeEnum::IntType(ctx.i16_type()),
            Self::Float => BasicTypeEnum::FloatType(ctx.f32_type()),
            Self::Double => BasicTypeEnum::FloatType(ctx.f64_type()),
            Self::Reference => BasicTypeEnum::PointerType(ctx.i128_type().ptr_type(AddressSpace::Generic)), // For alignment
            Self::Void => panic!("llvm type of void"),
            Self::Top => panic!("llvm type of top"),
        }
    }
}