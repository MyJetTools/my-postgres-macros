use proc_macro2::Ident;

pub struct ETagData<'s> {
    pub column_name: &'s str,
    pub field_name: &'s Ident,
}

pub fn generate_e_tag_methods(e_tag_data: Option<ETagData>) -> proc_macro2::TokenStream {
    let get_e_tag_field_name_body: proc_macro2::TokenStream;
    let get_e_tag_value_body: proc_macro2::TokenStream;
    let set_e_tag_value_body: proc_macro2::TokenStream;

    if let Some(e_tag_data) = e_tag_data {
        let column_name = e_tag_data.column_name;
        let field_name = e_tag_data.field_name;

        get_e_tag_field_name_body = quote::quote! {
            Some(#column_name)
        };

        get_e_tag_value_body = quote::quote! {
            Some(*self.#field_name)
        };

        set_e_tag_value_body = quote::quote! {
            let reference = &self.#field_name;

            unsafe {
                let mutable_reference = reference as *mut i64;
                *mutable_reference = value;
            }
        };
    } else {
        get_e_tag_field_name_body = quote::quote!(None);
        get_e_tag_value_body = quote::quote!(None);
        set_e_tag_value_body = quote::quote! {};
    }

    quote::quote! {
        fn get_e_tag_field_name() -> Option<&'static str>{
            #get_e_tag_field_name_body
        }
        fn get_e_tag_value(&self) -> Option<i64>{
            #get_e_tag_value_body
        }
        fn set_e_tag_value(&self, value: i64){
            #set_e_tag_value_body
        }
    }
}
