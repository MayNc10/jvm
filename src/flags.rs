// TODO: Implment BitAnd for these types.
pub mod class {
    use std::fmt;

    pub const ACC_PUBLIC: u16 = 0x0001;
    pub const ACC_FINAL: u16 = 0x0010;
    pub const ACC_SUPER: u16 = 0x0020;
    pub const ACC_INTERFACE: u16 = 0x0200;
    pub const ACC_ABSTRACT: u16 = 0x0400;
    pub const ACC_SYNTHETIC: u16 = 0x1000;
    pub const ACC_ANNOTATION: u16 = 0x2000;
    pub const ACC_ENUM: u16 = 0x4000;
    pub const ACC_MODULE: u16 = 0x8000;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct AccessFlags {
        pub flags: u16
    }

    impl fmt::Display for AccessFlags {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut text = String::new();
            if (self.flags & ACC_PUBLIC) > 0 {
                text.push_str(", ACC_PUBLIC");
            }
            if (self.flags & ACC_FINAL) > 0 {
                text.push_str(", ACC_FINAL");
            }
            if (self.flags & ACC_SUPER) > 0 {
                text.push_str(", ACC_SUPER");
            }
            if (self.flags & ACC_INTERFACE) > 0 {
                text.push_str(", ACC_INTERFACE");
            }
            if (self.flags & ACC_ABSTRACT) > 0 {
                text.push_str(", ACC_ABSTRACT");
            }
            if (self.flags & ACC_SYNTHETIC) > 0 {
                text.push_str(", ACC_SYNTHETIC");
            }
            if (self.flags & ACC_ANNOTATION) > 0 {
                text.push_str(", ACC_ANNOTATION");
            }
            if (self.flags & ACC_ENUM) > 0 {
                text.push_str(", ACC_ENUM");
            }
            if (self.flags & ACC_MODULE) > 0 {
                text.push_str(", ACC_MODULE");
            }
            if text.len() > 0 {
                text.remove(0);
                text.remove(0);
            }
            write!(f, "{}", text)
        }
    }
}

pub mod field {
    use std::fmt;

    pub const ACC_PUBLIC: u16 = 0x0001;
    pub const ACC_PRIVATE: u16 = 0x0002;
    pub const ACC_PROTECTED: u16 = 0x0004;
    pub const ACC_STATIC: u16 = 0x0008;
    pub const ACC_FINAL: u16 = 0x0010;
    pub const ACC_VOLATILE: u16 = 0x0040;
    pub const ACC_TRANSIENT: u16 = 0x0080;
    pub const ACC_SYNTHETIC: u16 = 0x1000;
    pub const ACC_ENUM: u16 = 0x4000;

    #[derive(Clone, Debug, PartialEq)]
    pub struct AccessFlags {
        pub flags: u16
    }

    impl fmt::Display for AccessFlags {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut text = String::new();
            if (self.flags & ACC_PUBLIC) > 0 {
                text.push_str(", ACC_PUBLIC");
            }
            if (self.flags & ACC_PRIVATE) > 0 {
                text.push_str(", ACC_PRIVATE");
            }
            if (self.flags & ACC_PROTECTED) > 0 {
                text.push_str(", ACC_PROTECTED");
            }
            if (self.flags & ACC_STATIC) > 0 {
                text.push_str(", ACC_STATIC");
            }
            if (self.flags & ACC_FINAL) > 0 {
                text.push_str(", ACC_FINAL");
            }
            if (self.flags & ACC_VOLATILE) > 0 {
                text.push_str(", ACC_MODULE");
            }
            if (self.flags & ACC_TRANSIENT) > 0 {
                text.push_str(", ACC_TRANSIENT");
            }
            if (self.flags & ACC_SYNTHETIC) > 0 {
                text.push_str(", ACC_SYNTHETIC");
            }
            if (self.flags & ACC_ENUM) > 0 {
                text.push_str(", ACC_ENUM");
            }
            if text.len() > 0 {
                text.remove(0);
                text.remove(0);
            }
            write!(f, "{}", text)
        }
    }
}

pub mod method {
    use std::fmt;

    pub const ACC_PUBLIC: u16 = 0x0001;
    pub const ACC_PRIVATE: u16 = 0x0002;
    pub const ACC_PROTECTED: u16 = 0x0004;
    pub const ACC_STATIC: u16 = 0x0008;
    pub const ACC_FINAL: u16 = 0x0010;
    pub const ACC_SYNCHRONIZED: u16 = 0x0020;
    pub const ACC_BRIDGE: u16 = 0x0040;
    pub const ACC_VARARGS: u16 = 0x0080;
    pub const ACC_NATIVE: u16 = 0x0100;
    pub const ACC_ABSTRACT: u16 = 0x0400;
    pub const ACC_STRICT: u16 = 0x0800;
    pub const ACC_SYNTHETIC: u16 = 0x1000;

    #[derive(Clone, Debug, PartialEq)]
    pub struct AccessFlags {
        pub flags: u16
    }

    impl fmt::Display for AccessFlags {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut text = String::new();
            if (self.flags & ACC_PUBLIC) > 0 {
                text.push_str(", ACC_PUBLIC");
            }

            if (self.flags & ACC_PRIVATE) > 0 {
                text.push_str(", ACC_PRIVATE");
            }

            if (self.flags & ACC_PROTECTED) > 0 {
                text.push_str(", ACC_PROTECTED");
            }

            if (self.flags & ACC_STATIC) > 0 {
                text.push_str(", ACC_STATIC");
            }

            if (self.flags & ACC_FINAL) > 0 {
                text.push_str(", ACC_FINAL");
            }  
            if (self.flags & ACC_SYNCHRONIZED) > 0 {
                text.push_str(", ACC_SYNCHRONIZED");
            }
            if (self.flags & ACC_BRIDGE) > 0 {
                text.push_str(", ACC_BRIDGE");
            }
            if (self.flags & ACC_VARARGS) > 0 {
                text.push_str(", ACC_VARARGS");
            }
            if (self.flags & ACC_NATIVE) > 0 {
                text.push_str(", ACC_NATIVE");
            }
            if (self.flags & ACC_ABSTRACT) > 0 {
                text.push_str(", ACC_ABSTRACT");
            }
            if (self.flags & ACC_STRICT) > 0 {
                text.push_str(", ACC_STRICT");
            }
            if (self.flags & ACC_SYNTHETIC) > 0 {
                text.push_str(", ACC_SYNTHETIC");
            }
            if text.len() > 0 {
                text.remove(0);
                text.remove(0);
            }
            write!(f, "{}", text)
        }
    }
}