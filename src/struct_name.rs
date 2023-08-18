use std::str::FromStr;

use proc_macro2::TokenStream;
use types_reader::TypeName;

pub enum StructName<'s> {
    TypeName(&'s TypeName<'s>),
    TokenStream(&'s TokenStream),
}

impl<'s> StructName<'s> {
    pub fn get_struct_name(&self) -> TokenStream {
        match self {
            Self::TypeName(type_name) => type_name.get_type_name(),
            Self::TokenStream(type_name) => {
                quote::quote! {
                    #type_name
                }
            }
        }
    }
    pub fn get_generic(&self) -> TokenStream {
        match self {
            Self::TypeName(type_name) => {
                if let Some(generics) = type_name.generics {
                    TokenStream::from_str("<'s>").unwrap()
                } else {
                    TokenStream::from_str("").unwrap()
                }
            }
            Self::TokenStream(_) => TokenStream::from_str("").unwrap(),
        }
    }
}
