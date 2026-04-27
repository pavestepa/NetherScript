#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinTypeKey {
    Boolean,
    String,
    Void,
    Never,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    F32,
    F64,
    Char,
    Str,
}

#[derive(Debug, Clone, Copy)]
pub struct BuiltinTypeSpec {
    pub key: BuiltinTypeKey,
    pub name: &'static str,
}

pub const BUILTIN_TYPE_CATALOG: &[BuiltinTypeSpec] = &[
    BuiltinTypeSpec {
        key: BuiltinTypeKey::Boolean,
        name: "boolean",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::String,
        name: "String",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::Void,
        name: "void",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::Never,
        name: "never",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::I8,
        name: "i8",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::I16,
        name: "i16",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::I32,
        name: "i32",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::I64,
        name: "i64",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::I128,
        name: "i128",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::Isize,
        name: "isize",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::U8,
        name: "u8",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::U16,
        name: "u16",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::U32,
        name: "u32",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::U64,
        name: "u64",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::U128,
        name: "u128",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::Usize,
        name: "usize",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::F32,
        name: "f32",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::F64,
        name: "f64",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::Char,
        name: "char",
    },
    BuiltinTypeSpec {
        key: BuiltinTypeKey::Str,
        name: "str",
    },
];
