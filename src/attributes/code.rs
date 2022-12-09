use crate::jvm::instructions::Instruction;

use super::{annotations::TypeAnnotation};

pub mod stack_map_table {
    use crate::{errorcodes::Error, llvm::valuemarker::ValueMarker};

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
        /// # Safety
        ///  
        /// This function is unsafe because the caller has to guarantee that valid bytes exist at ```data_ptr```
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
                    eprintln!("Found VerificationType {any}");
                    Err(Error::IllegalVerificationType)
                }
            }
        }
        pub fn as_value_marker(&self) -> Option<ValueMarker> {
            match self {
                VerificationTypeInfo::Top => None,
                VerificationTypeInfo::Integer => Some(ValueMarker::Int),
                VerificationTypeInfo::Float => Some(ValueMarker::Float),
                VerificationTypeInfo::Long => Some(ValueMarker::Long),
                VerificationTypeInfo::Double => Some(ValueMarker::Double),
                _ => Some(ValueMarker::Reference),
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

    pub fn local_var_layout(map: &Vec<StackMapFrame>) -> Option<Vec<Option<ValueMarker>>> {
        let mut uncompressed_map: Vec<Vec<VerificationTypeInfo>> = Vec::new();
        for frame in map {
            match frame {
                StackMapFrame::SameFrame(_) | StackMapFrame::SameFrameExtended(_) | 
                StackMapFrame::SameLocals1StackItem(_, _) | StackMapFrame::SameLocals1StackItemExtended(_, _)
                 => uncompressed_map.push(uncompressed_map.last().unwrap().clone()),
                StackMapFrame::ChopFrame(ftype, _) => {
                    let last_map = uncompressed_map.last().unwrap();
                    uncompressed_map.push(last_map[0..last_map.len() - (251 - ftype) as usize].into())
                },
                StackMapFrame::AppendFrame(_, _, new_locals) => {
                    let mut new_map = uncompressed_map.last().unwrap().clone();
                    new_map.extend_from_slice(&new_locals);
                    uncompressed_map.push(new_map);
                },
                StackMapFrame::FullFrame(_, locals, _) => uncompressed_map.push(locals.clone()),
            }
        }
        let mut true_map = Vec::new();
        let mut idx = 0;
        loop {
            let mut found = false;
            for vec in &uncompressed_map {
                if idx < vec.len() {
                    if idx == true_map.len() {
                        true_map.push(vec[idx].clone().as_value_marker());
                    } 
                    else {
                        if true_map[idx] != vec[idx].as_value_marker() {
                            return None;
                        }
                    }
                    found = true;
                }
            }
            if !found {
                break;
            }
            idx += 1;
        }
        return Some(true_map);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocalVariableType {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Clone, PartialEq)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<Box<dyn Instruction>>,
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

use std::fmt;
impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Max Stack: {}", self.max_stack)?;
        writeln!(f, "Max Locals: {}", self.max_locals)?;
        writeln!(f, "Code Length: {}", self.code.len())?;
        writeln!(f, "Code:")?;
        let mut op_idx = 0;
        for op in &self.code {
            writeln!(f, "   {} idx: {}", op_idx, op)?;
            op_idx += 1;
        }
        writeln!(f, "Exception Table Size: {}", self.exception_table.len())?;
        writeln!(f, "Exception Table:")?;
        for e in &self.exception_table {
            writeln!(f, "{e:#?}")?;
        }
        writeln!(f, "Line Number Table Size: {}", self.line_number_table.len())?;
        writeln!(f, "Line Number Table:")?;
        for line in &self.line_number_table {
            writeln!(f, "{line:#?}")?;
        }
        writeln!(f, "Local Variable Table Size: {}", self.local_variable_table.len())?;
        writeln!(f, "Local Variable Table:")?;
        for local in &self.local_variable_table {
            writeln!(f, "{local:#?}")?;
        }
        writeln!(f, "Local Variable Type Table Size: {}", self.local_variable_type_table.len())?;
        writeln!(f, "Local Variable Type Table:")?;
        for local in &self.local_variable_type_table {
            writeln!(f, "{local:#?}")?;
        }
        if self.stack_map_table.is_some() {
            writeln!(f, "Stack Map Table Size: {}", self.stack_map_table.as_ref().unwrap().len())?;
            writeln!(f, "Stack Map Table:")?;
            for frame in self.stack_map_table.as_ref().unwrap() {
                writeln!(f, "{frame:#?}")?;
            }
        }
        else {
            writeln!(f, "This Code has no Stack Map Table")?;
        }
        if self.rt_vis_type_annotations.is_some() {
            writeln!(f, "Number of Runtime Visisble Type Annotation: {}", self.rt_vis_type_annotations.as_ref().unwrap().len())?;
            writeln!(f, "Runtime Visisble Type Annotation:")?;
            for annotation in self.rt_vis_type_annotations.as_ref().unwrap() {
                writeln!(f, "{annotation:#?}")?;
            }
        }
        else {
            writeln!(f, "This Code has no Runtime Visisble Type Annotation Table")?;
        }
        if self.rt_vis_type_annotations.is_some() {
            writeln!(f, "Number of Runtime Invisisble Type Annotation: {}", self.rt_invis_type_annotations.as_ref().unwrap().len())?;
            write!(f, "Runtime Invisisble Type Annotation:")?;
            for annotation in self.rt_invis_type_annotations.as_ref().unwrap() {
                write!(f, "\n{annotation:#?}")?;
            }
        }
        else {
            write!(f, "This Code has no Runtime Invisisble Type Annotation Table")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}