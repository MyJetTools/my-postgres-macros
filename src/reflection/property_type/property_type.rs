/*
pub struct PropertyType {
    pub type_name: String,
    pub tp: syn::TypePath,
}
 */

use syn::TypePath;

pub const U8: &str = "u8";
pub const I8: &str = "i8";
pub const U16: &str = "u16";
pub const I16: &str = "i16";
pub const U32: &str = "u32";
pub const I32: &str = "i32";
pub const U64: &str = "u64";
pub const I64: &str = "i64";
pub const F32: &str = "f32";
pub const F64: &str = "f64";
pub const U_SIZE: &str = "usize";
pub const I_SIZE: &str = "isize";
pub const BOOL: &str = "bool";
pub const STRING: &str = "String";
pub const DATETIME: &str = "DateTimeAsMicroseconds";

#[derive(Debug)]
pub enum PropertyType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    USize,
    ISize,
    String,
    Str,
    Bool,
    DateTime,
    OptionOf(Box<PropertyType>),
    VecOf(Box<PropertyType>),
    Struct(String),
}

impl PropertyType {
    pub fn new(field: &syn::Field) -> Self {
        match &field.ty {
            syn::Type::Slice(_) => panic!("Slice type is not supported"),
            syn::Type::Array(_) => panic!("Array type is not supported"),
            syn::Type::Ptr(_) => panic!("Ptr type is not supported"),
            syn::Type::Reference(_) => PropertyType::Str,
            syn::Type::BareFn(_) => panic!("BareFn type is not supported"),
            syn::Type::Never(_) => panic!("Never type is not supported"),
            syn::Type::Tuple(_) => panic!("Tuple type is not supported"),
            syn::Type::Path(type_path) => {
                let type_as_string = super::utils::simple_type_to_string(type_path);
                return PropertyType::parse(type_as_string.as_str(), type_path);
            }
            syn::Type::TraitObject(_) => panic!("TraitObject type is not supported"),
            syn::Type::ImplTrait(_) => panic!("ImplTrait type is not supported"),
            syn::Type::Paren(_) => panic!("Paren type is not supported"),
            syn::Type::Group(_) => panic!("Group type is not supported"),
            syn::Type::Infer(_) => panic!("Infer type is not supported"),
            syn::Type::Macro(_) => panic!("Macro type is not supported"),
            syn::Type::Verbatim(_) => panic!("Verbatim type is not supported"),
            _ => panic!("{:?} type is not supported", &field.ty),
        }
    }

    pub fn parse(src: &str, type_path: &TypePath) -> Self {
        match src {
            U8 => PropertyType::U8,
            I8 => PropertyType::I8,
            U16 => PropertyType::U16,
            I16 => PropertyType::I16,
            U32 => PropertyType::U32,
            I32 => PropertyType::I32,
            U64 => PropertyType::U64,
            I64 => PropertyType::I64,
            F32 => PropertyType::F32,
            F64 => PropertyType::F64,
            U_SIZE => PropertyType::USize,
            I_SIZE => PropertyType::ISize,
            BOOL => PropertyType::Bool,
            STRING => PropertyType::String,
            DATETIME => PropertyType::DateTime,
            "Option" => PropertyType::OptionOf(Box::new(super::utils::get_generic(type_path))),
            "Vec" => PropertyType::VecOf(Box::new(super::utils::get_generic(type_path))),
            _ => PropertyType::Struct(src.to_string()),
        }
    }

    pub fn is_date_time(&self) -> bool {
        if let PropertyType::DateTime = self {
            return true;
        }

        false
    }

    pub fn is_string(&self) -> bool {
        if let PropertyType::String = self {
            return true;
        }

        false
    }
}
