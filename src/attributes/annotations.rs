use crate::{errorcodes::{Error, Opcode}, data_access};

#[derive(Clone, Debug, PartialEq)]
pub struct Annotation {
    type_index: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

impl Annotation {
    pub unsafe fn new(data: *const u8, location: &mut isize) -> Result<Annotation, Error> {
        let type_index = data_access::read_u16(data, location);
        let num_pairs = data_access::read_u16(data, location);
        let mut ev_pairs = Vec::with_capacity(num_pairs as usize);
        while ev_pairs.capacity() > ev_pairs.len() {
            ev_pairs.push(ElementValuePair {
                element_name_index: data_access::read_u16(data, location),
                value: ElementValue::new(data, location)?,
            })
        }
        Ok(Annotation {
            type_index,
            element_value_pairs: ev_pairs,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ElementValuePair {
    element_name_index: u16,
    value: ElementValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ElementValue {
    ConstantValue(Constant), 
    EnumConstantValue(Enum),
    ClassInfo(u16),
    AnnotationValue(Annotation),
    ArrayValue(Vec<ElementValue>),
}

impl ElementValue {
    pub unsafe fn new(data: *const u8, location: &mut isize) -> Result<ElementValue, Error> {
        let tag = data_access::read_u8(data, location);
        match tag as char {
            'B' | 'C' | 'I' | 'S' | 'Z'  => Ok(ElementValue::ConstantValue(Constant::Integer(data_access::read_u16(data, location)))),
            'D' => Ok(ElementValue::ConstantValue(Constant::Double(data_access::read_u16(data, location)))),
            'F' => Ok(ElementValue::ConstantValue(Constant::Float(data_access::read_u16(data, location)))),
            'J' => Ok(ElementValue::ConstantValue(Constant::Long(data_access::read_u16(data, location)))),
            's' => Ok(ElementValue::ConstantValue(Constant::Utf8(data_access::read_u16(data, location)))),
            'e' => Ok(ElementValue::EnumConstantValue(Enum {
                type_name_index: data_access::read_u16(data, location),
                const_name_index: data_access::read_u16(data, location),
            })),
            'c' => Ok(ElementValue::ClassInfo(data_access::read_u16(data, location))),
            '@' => Ok(ElementValue::AnnotationValue(Annotation::new(data, location)?)),
            '[' => {
                let num_values = data_access::read_u16(data, location);
                let mut values = Vec::with_capacity(num_values as usize);
                while values.capacity() > values.len() {
                    values.push(ElementValue::new(data, location)?);
                }
                Ok(ElementValue::ArrayValue(values))
            },
            _ => Err(Error::IllegalElementValueTag)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    Integer(u16),
    Double(u16),
    Float(u16),
    Long(u16),
    Utf8(u16),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enum {
    type_name_index: u16,
    const_name_index: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAnnotation {
    target_info: Target,
    target_path: Vec<Path>,
    type_index: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

impl TypeAnnotation {
    pub unsafe fn new(data: *const u8, location: &mut isize) -> Result<TypeAnnotation, Error> {
        let target_type = data_access::read_u8(data, location);
        Err(Error::Todo(Opcode::AALOAD)) // Just a placeholder, TODO.
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    TypeParameter(u8),
    SuperType(u16),
    TypeParameterBound(Bound),
    Empty,
    FormalParameter(u8),
    Throws(u16),
    LocalVar(Vec<LocalVar>),
    Catch(u16),
    Offset(u16),
    TypeArguement(Argument),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Bound {
    type_parameter_index: u8,
    bound_index: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalVar {
    start_pc: u16,
    length: u16,
    index: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Argument {
    offset: u16,
    type_arguement_index: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Path {
    DeeperArray(u8),
    DeeperNestedType(u8),
    BoundedWildcardTypeArg(u8),
    TypeArg(u8),
}

