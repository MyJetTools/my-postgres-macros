use proc_macro::TokenStream;
use quote::quote;
use types_reader::StructProperty;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = StructProperty::read(ast);

    let line_no_prop = fn_select_line_no(&fields);

    quote! {
        impl my_postgres::sql_select::BulkSelectEntity for #name{
            fn get_line_no(&self) -> i32 {
                #line_no_prop
            }
        }
    }
    .into()
}

fn fn_select_line_no(struct_properties: &[StructProperty]) -> proc_macro2::TokenStream {
    for struct_property in struct_properties {
        if struct_property.attrs.contains_key("line_no") || struct_property.name == "line_no" {
            let prop_name = struct_property.get_field_name_ident();
            return quote! {
                self.#prop_name
            }
            .into();
        }
    }

    panic!("line_no not found");
}
