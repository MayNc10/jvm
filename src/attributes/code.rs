use super::{annotations::TypeAnnotation};

pub mod stack_map_table {
    use crate::errorcodes::Error;

    #[derive(Clone, Debug, PartialEq)]
    pub enum VerificationTypeInfo {
        Top,
        Integer,
        Float,
        Null,
        UninitializedThis,
        Object(u16),
        Uninitialized(u16),
        Long,
        Double,
    }

    impl VerificationTypeInfo {
        pub unsafe fn new(data_ptr: *const u8, location: &mut isize) -> Result<VerificationTypeInfo, Error> {
            match crate::data_access::read_u8(data_ptr, location) {
                0 => Ok(VerificationTypeInfo::Top),
                1 => Ok(VerificationTypeInfo::Integer),
                2 => Ok(VerificationTypeInfo::Float),
                3 => Ok(VerificationTypeInfo::Double),
                4 => Ok(VerificationTypeInfo::Long),
                5 => Ok(VerificationTypeInfo::Null),
                6 => Ok(VerificationTypeInfo::UninitializedThis),
                7 => Ok(VerificationTypeInfo::Object(crate::data_access::read_u16(data_ptr, location))),
                8 => Ok(VerificationTypeInfo::Uninitialized(crate::data_access::read_u16(data_ptr, location))),
                any => {
                    println!("Found VerificationType {}", any);
                    Err(Error::IllegalVerificationType)
                }
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum StackMapFrame {
        SameFrame(u8),
        SameLocals1StackItem(u8, VerificationTypeInfo),
        SameLocals1StackItemExtended(u16, VerificationTypeInfo),
        ChopFrame(u8, u16),
        SameFrameExtended(u16),
        AppendFrame(u8, u16, Vec<VerificationTypeInfo>),
        FullFrame(u16, Vec<VerificationTypeInfo>, Vec<VerificationTypeInfo>),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalVariableType {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<Exception>,
    // A couple of these attributes can appear in multiples, and so we keep trakc of the total number using a Vec<Vec<>>.
    // This isn't neccesary for line number table, even though it can have multiples, so we just use one Vec<>.
    pub line_number_table: Vec<LineNumber>,
    pub local_variable_table: Vec<Vec<LocalVariable>>,
    pub local_variable_type_table: Vec<Vec<LocalVariableType>>,
    pub stack_map_table: Option<Vec<stack_map_table::StackMapFrame>>,
    pub rt_vis_type_annotations: Option<Vec<TypeAnnotation>>,
    pub rt_invis_type_annotations: Option<Vec<TypeAnnotation>>,
}