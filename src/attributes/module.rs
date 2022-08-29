#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    pub module_name_index: u16,
    pub module_flags: u16,
    pub module_version_index: u16,
    pub requires: Vec<Require>,
    pub exports: Vec<Export>,
    pub opens: Vec<Open>,
    pub uses: Vec<u16>,
    pub provides: Vec<Provide>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Require {
    pub requires_index: u16,
    pub requires_flags: u16,
    pub requires_version_count: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Export {
    pub exports_index: u16,
    pub exports_flags: u16,
    pub exports_to_index: Vec<u16>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Open {
    pub opens_index: u16,
    pub opens_flags: u16,
    pub opens_to_index: Vec<u16>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Provide {
    pub provides_index: u16,
    pub provides_with_index: Vec<u16>,
}