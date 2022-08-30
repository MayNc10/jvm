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

#[derive(Clone, PartialEq)]
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

use std::fmt;
impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Max Stack: {}", self.max_stack)?;
        writeln!(f, "Max Locals: {}", self.max_locals)?;
        let mut code_index = 0;
        writeln!(f, "Code Length: {}", self.code.len())?;
        writeln!(f, "Code:")?;
        while code_index < self.code.len() {
            write!(f, "    ")?;
            match self.code[code_index] {
                // TODO: WIDE  
                0 => writeln!(f, "nop")?,
                1 => writeln!(f, "aconst_null")?,
                2 => writeln!(f, "iconst_m1")?,
                3 => writeln!(f, "iconst_0")?,
                4 => writeln!(f, "iconst_1")?,
                5 => writeln!(f, "iconst_2")?,
                6 => writeln!(f, "iconst_3")?,
                7 => writeln!(f, "iconst_4")?,
                8 => writeln!(f, "iconst_5")?,
                9 => writeln!(f, "lconst_0")?,
                10 => writeln!(f, "lconst_1")?,
                11 => writeln!(f, "fconst_0")?,
                12 => writeln!(f, "fconst_1")?,
                13 => writeln!(f, "fconst_2")?,
                14 => writeln!(f, "dconst_0")?,
                15 => writeln!(f, "dconst_1")?,
                16 => {
                    writeln!(f, "bipush {}", self.code[code_index + 1])?;
                    code_index += 1;
                },
                17 => {
                    let num = {
                        let b1 = self.code[code_index + 1] as u16;
                        let b2 = self.code[code_index + 2] as u16;
                        b1 << 8 + b2
                    };
                    writeln!(f, "sipush {}", num)?;
                    code_index += 2;
                },
                18 => {
                    writeln!(f, "ldc #{}", self.code[code_index + 1])?;
                    code_index += 1;
                },
                19 => {
                    let num = {
                        let b1 = self.code[code_index + 1] as u16;
                        let b2 = self.code[code_index + 2] as u16;
                        b1 << 8 + b2
                    };
                    writeln!(f, "ldc_w #{}", num)?;
                    code_index += 2;
                },
                20 => {
                    let num = {
                        let b1 = self.code[code_index + 1] as u16;
                        let b2 = self.code[code_index + 2] as u16;
                        b1 << 8 + b2
                    };
                    writeln!(f, "ldc2_w #{}", num)?;
                    code_index += 2;
                },
                21 => {
                    writeln!(f, "iload #{}", self.code[code_index + 1])?;
                    code_index += 1;
                },
                22 => {
                    writeln!(f, "lload #{}", self.code[code_index + 1])?;
                    code_index += 1;
                },
                23 => {
                    writeln!(f, "fload #{}", self.code[code_index + 1])?;
                    code_index += 1;
                },
                24 => {
                    writeln!(f, "dload #{}", self.code[code_index + 1])?;
                    code_index += 1;
                },
                25 => {
                    writeln!(f, "aload #{}", self.code[code_index + 1])?;
                    code_index += 1;
                },
                26 => writeln!(f, "iload_0")?,
                27 => writeln!(f, "iload_1")?,
                28 => writeln!(f, "iload_2")?,
                29 => writeln!(f, "iload_3")?,
                30 => writeln!(f, "lload_0")?,
                31 => writeln!(f, "lload_1")?,
                32 => writeln!(f, "lload_2")?,
                33 => writeln!(f, "lload_3")?,
                34 => writeln!(f, "fload_0")?,
                35 => writeln!(f, "fload_1")?,
                36 => writeln!(f, "fload_2")?,
                37 => writeln!(f, "fload_3")?,
                38 => writeln!(f, "dload_0")?,
                39 => writeln!(f, "dload_1")?,
                40 => writeln!(f, "dload_2")?,
                41 => writeln!(f, "dload_3")?,
                42 => writeln!(f, "aload_0")?,
                43 => writeln!(f, "aload_1")?,
                44 => writeln!(f, "aload_2")?,
                45 => writeln!(f, "aload_3")?,
                46 => writeln!(f, "iaload")?,
                47 => writeln!(f, "laload")?,
                48 => writeln!(f, "faload")?,
                49 => writeln!(f, "daload")?,
                50 => writeln!(f, "aaload")?,
                51 => writeln!(f, "baload")?,
                52 => writeln!(f, "caload")?,
                53 => writeln!(f, "saload")?,
                54 => {
                    writeln!(f, "istore #{}", self.code[code_index + 1])?;
                    code_index += 1;
                },
                55 => {
                    writeln!(f, "lstore #{}", self.code[code_index + 1])?;
                    code_index += 1;
                },
                56 => {
                    writeln!(f, "fstore #{}", self.code[code_index + 1])?;
                    code_index += 1;
                },
                57 => {
                    writeln!(f, "dstore #{}", self.code[code_index + 1])?;
                    code_index += 1;
                },
                58 => {
                    writeln!(f, "astore #{}", self.code[code_index + 1])?;
                    code_index += 1;
                },
                59 => writeln!(f, "istore_0")?,
                60 => writeln!(f, "istore_1")?,
                61 => writeln!(f, "istore_2")?,
                62 => writeln!(f, "istore_3")?,
                63 => writeln!(f, "lstore_0")?,
                64 => writeln!(f, "lstore_1")?,
                65 => writeln!(f, "lstore_2")?,
                66 => writeln!(f, "lstore_3")?,
                67 => writeln!(f, "fstore_0")?,
                68 => writeln!(f, "fstore_1")?,
                69 => writeln!(f, "fstore_2")?,
                70 => writeln!(f, "fstore_3")?,
                71 => writeln!(f, "dstore_0")?,
                72 => writeln!(f, "dstore_1")?,
                73 => writeln!(f, "dstore_2")?,
                74 => writeln!(f, "dstore_3")?,
                75 => writeln!(f, "astore_0")?,
                76 => writeln!(f, "astore_1")?,
                77 => writeln!(f, "astore_2")?,
                78 => writeln!(f, "astore_3")?,
                79 => writeln!(f, "iastore")?,
                80 => writeln!(f, "lastore")?,
                81 => writeln!(f, "fastore")?,
                82 => writeln!(f, "dastore")?,
                83 => writeln!(f, "aastore")?,
                84 => writeln!(f, "bastore")?,
                85 => writeln!(f, "castore")?,
                86 => writeln!(f, "sastore")?,
                87 => writeln!(f, "pop")?,
                88 => writeln!(f, "pop2")?,
                89 => writeln!(f, "dup")?,
                90 => writeln!(f, "dup2")?,
                91 => writeln!(f, "dup_x1")?,
                92 => writeln!(f, "dup_x2")?,
                93 => writeln!(f, "dup2_x1")?,
                94 => writeln!(f, "dup2_x2")?,
                95 => writeln!(f, "swap")?,
                96 => writeln!(f, "iadd")?,
                97 => writeln!(f, "ladd")?,
                98 => writeln!(f, "fadd")?,
                99 => writeln!(f, "dadd")?,
                100 => writeln!(f, "isub")?,
                101 => writeln!(f, "lsub")?,
                102 => writeln!(f, "fsub")?,
                103 => writeln!(f, "dsub")?,
                104 => writeln!(f, "imul")?,
                105 => writeln!(f, "lmul")?,
                106 => writeln!(f, "fmul")?,
                107 => writeln!(f, "dmul")?,
                108 => writeln!(f, "idiv")?,
                109 => writeln!(f, "ldiv")?,
                110 => writeln!(f, "fdiv")?,
                111 => writeln!(f, "ddiv")?,
                112 => writeln!(f, "irem")?,
                113 => writeln!(f, "lrem")?,
                114 => writeln!(f, "frem")?,
                115 => writeln!(f, "drem")?,
                116 => writeln!(f, "ineg")?,
                117 => writeln!(f, "lneg")?,
                118 => writeln!(f, "fneg")?,
                119 => writeln!(f, "dneg")?,
                120 => writeln!(f, "ishl")?,
                121 => writeln!(f, "lshl")?,
                122 => writeln!(f, "ishr")?,
                123 => writeln!(f, "lshr")?,
                124 => writeln!(f, "iushr")?,
                125 => writeln!(f, "lushr")?,
                126 => writeln!(f, "iand")?,
                127 => writeln!(f, "land")?,
                128 => writeln!(f, "ior")?,
                129 => writeln!(f, "lor")?,
                130 => writeln!(f, "ixor")?,
                131 => writeln!(f, "lxor")?,
                132 => {
                    let index = self.code[code_index + 1];
                    let const_ = self.code[code_index + 2];
                    writeln!(f, "iinc #{} {}", index, const_)?;
                    code_index += 2;
                },
                133 => writeln!(f, "i2l")?,
                134 => writeln!(f, "i2f")?,
                135 => writeln!(f, "i2d")?,
                136 => writeln!(f, "l2i")?,
                137 => writeln!(f, "l2f")?,
                138 => writeln!(f, "l2d")?,
                139 => writeln!(f, "f2i")?,
                140 => writeln!(f, "f2l")?,
                141 => writeln!(f, "f2d")?,
                142 => writeln!(f, "d2i")?,
                143 => writeln!(f, "d2l")?,
                144 => writeln!(f, "d2f")?,
                145 => writeln!(f, "i2b")?,
                146 => writeln!(f, "i2c")?,
                147 => writeln!(f, "i2s")?,
                148 => writeln!(f, "lcmp")?,
                149 => writeln!(f, "fcmpl")?,
                150 => writeln!(f, "fcmpg")?,
                151 => writeln!(f, "dcmpl")?,
                152 => writeln!(f, "dcmpg")?,
                153 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "ifeq {}", branch)?;
                    code_index += 2;
                },
                154 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "ifne {}", branch)?;
                    code_index += 2;
                },
                155 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "iflt {}", branch)?;
                    code_index += 2;
                },
                156 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "ifge {}", branch)?;
                    code_index += 2;
                },
                157 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "ifgt {}", branch)?;
                    code_index += 2;
                },
                158 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "ifle {}", branch)?;
                    code_index += 2;
                },
                159 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "if_icmpeq {}", branch)?;
                    code_index += 2;
                },
                160 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "if_icmpne {}", branch)?;
                    code_index += 2;
                },
                161 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "if_icmplt {}", branch)?;
                    code_index += 2;
                },
                162 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "if_icmpge {}", branch)?;
                    code_index += 2;
                },
                163 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "if_icmpgt {}", branch)?;
                    code_index += 2;
                },
                164 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "if_icmple {}", branch)?;
                    code_index += 2;
                },
                165 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "if_acmpeq {}", branch)?;
                    code_index += 2;
                },
                166 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "if_acmpne {}", branch)?;
                    code_index += 2;
                },
                167 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "goto {}", branch)?;
                    code_index += 2;
                },
                168 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "jsr {}", branch)?;
                    code_index += 2;
                },
                169 => {     
                    let index = self.code[code_index + 1];                        
                    writeln!(f, "ret #{}", index)?;
                    code_index += 2;
                },
                170 => {
                    panic!("TODO tableswitch");
                }
                171 => {
                    panic!("TODO lookupswitch");
                }
                172 => writeln!(f, "ireturn")?,
                173 => writeln!(f, "lreturn")?,
                174 => writeln!(f, "freturn")?,
                175 => writeln!(f, "dreturn")?,
                176 => writeln!(f, "areturn")?,
                177 => writeln!(f, "return")?,
                178 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "getstatic #{}", index)?;
                    code_index += 2;
                },
                179 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "putstatic #{}", index)?;
                    code_index += 2;
                },
                180 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "getfield #{}", index)?;
                    code_index += 2;
                },
                181 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "putfield #{}", index)?;
                    code_index += 2;
                },
                182 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "invokevirtual #{}", index)?;
                    code_index += 2;
                },
                183 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "invokespecial #{}", index)?;
                    code_index += 2;
                },
                184 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "invokestatic #{}", index)?;
                    code_index += 2;
                },
                185 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "invokeinterface #{}", index)?;
                    code_index += 4;
                },
                186 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "invokedynamic #{}", index)?;
                    code_index += 4;
                },
                187 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "new #{}", index)?;
                    code_index += 2;
                },
                188 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "newarray #{}", index)?;
                    code_index += 2;
                },
                189 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "anewarray #{}", index)?;
                    code_index += 2;
                },
                190 => writeln!(f, "arraylength")?,
                191 => writeln!(f, "athrow")?,
                192 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "checkcast #{}", index)?;
                    code_index += 2;
                },
                193 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "instanceof #{}", index)?;
                    code_index += 2;
                },
                194 => writeln!(f, "monitorenter")?,
                195 => writeln!(f, "monitorexit")?,
                196 => writeln!(f, "wide")?, // TODO! ADD FLAG FOR WIDE
                197 => {
                    let index = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    let dimensions = self.code[code_index + 3];
                    writeln!(f, "multianewarray #{} {}", index, dimensions)?;
                    code_index += 3;
                },
                198 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "ifnull #{}", branch)?;
                    code_index += 2;
                },
                199 => {
                    let branch = unsafe {                      
                        u16::from_be_bytes(std::slice::from_raw_parts(self.code.as_ptr().add(code_index + 1), 2).try_into().unwrap())                      
                    };
                    writeln!(f, "ifnonnull #{}", branch)?;
                    code_index += 2;
                },
                200 => {
                    let branch = {
                        let b1 = self.code[code_index + 1] as u32;
                        let b2 = self.code[code_index + 2] as u32;
                        let b3 = self.code[code_index + 3] as u32;
                        let b4 = self.code[code_index + 4] as u32;
                        b1 << 24 + b2 << 16 + b3 << 8 + b4
                    };
                    writeln!(f, "goto_w {}", branch)?;
                    code_index += 4;
                },
                201 => {
                    let branch = {
                        let b1 = self.code[code_index + 1] as u32;
                        let b2 = self.code[code_index + 2] as u32;
                        let b3 = self.code[code_index + 3] as u32;
                        let b4 = self.code[code_index + 4] as u32;
                        b1 << 24 + b2 << 16 + b3 << 8 + b4
                    };
                    writeln!(f, "jsr_w {}", branch)?;
                    code_index += 4;
                },
                254 => writeln!(f, "imdep1")?,
                255 => writeln!(f, "imdep2")?,
                op => {
                    panic!("Error: Opcode {} not supported yet", op);
                },
            }
            code_index += 1;
        }
        writeln!(f, "Exception Table Size: {}", self.exception_table.len())?;
        writeln!(f, "Exception Table:")?;
        for e in &self.exception_table {
            writeln!(f, "{:#?}", e)?;
        }
        writeln!(f, "Line Number Table Size: {}", self.line_number_table.len())?;
        writeln!(f, "Line Number Table:")?;
        for line in &self.line_number_table {
            writeln!(f, "{:#?}", line)?;
        }
        writeln!(f, "Local Variable Table Size: {}", self.local_variable_table.len())?;
        writeln!(f, "Local Variable Table:")?;
        for local in &self.local_variable_table {
            writeln!(f, "{:#?}", local)?;
        }
        writeln!(f, "Local Variable Type Table Size: {}", self.local_variable_type_table.len())?;
        writeln!(f, "Local Variable Type Table:")?;
        for local in &self.local_variable_type_table {
            writeln!(f, "{:#?}", local)?;
        }
        if self.stack_map_table.is_some() {
            writeln!(f, "Stack Map Table Size: {}", self.stack_map_table.as_ref().unwrap().len())?;
            writeln!(f, "Stack Map Table:")?;
            for frame in self.stack_map_table.as_ref().unwrap() {
                writeln!(f, "{:#?}", frame)?;
            }
        }
        else {
            writeln!(f, "This Code has no Stack Map Table")?;
        }
        if self.rt_vis_type_annotations.is_some() {
            writeln!(f, "Number of Runtime Visisble Type Annotation: {}", self.rt_vis_type_annotations.as_ref().unwrap().len())?;
            writeln!(f, "Runtime Visisble Type Annotation:")?;
            for annotation in self.rt_vis_type_annotations.as_ref().unwrap() {
                writeln!(f, "{:#?}", annotation)?;
            }
        }
        else {
            writeln!(f, "This Code has no Runtime Visisble Type Annotation Table")?;
        }
        if self.rt_vis_type_annotations.is_some() {
            writeln!(f, "Number of Runtime Invisisble Type Annotation: {}", self.rt_invis_type_annotations.as_ref().unwrap().len())?;
            write!(f, "Runtime Invisisble Type Annotation:")?;
            for annotation in self.rt_invis_type_annotations.as_ref().unwrap() {
                write!(f, "\n{:#?}", annotation)?;
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
        write!(f, "{}", self)
    }
}