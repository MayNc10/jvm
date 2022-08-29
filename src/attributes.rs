use std::vec::Vec;

use self::{annotations::{Annotation, TypeAnnotation}};

pub mod annotations;
pub mod code;
pub mod module;

#[derive(Clone, Debug, PartialEq)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub bootstrap_arguments: Vec<u16>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnclosingMethod {
    pub class_index: u16,
    pub method_index: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodParameter {
    pub name_index: u16,
    pub access_flags: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordComponentInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub signature: Option<u16>,
    pub rt_vis_annotations: Option<Vec<Annotation>>,
    pub rt_invis_annotations: Option<Vec<Annotation>>,
    pub rt_vis_type_annotations: Option<Vec<TypeAnnotation>>,
    pub rt_invis_type_annotations: Option<Vec<TypeAnnotation>>,
}


