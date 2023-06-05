pub fn generate_e_tag_methods() -> proc_macro2::TokenStream {
    quote::quote! {
        fn get_e_tag_update_field_name() -> Option<&'static str>{
            None
        }
        fn get_e_tag_update_value(&self) -> Option<i64>{
            None
        }
        fn set_e_tag_update_value(&self, value: i64){
            None
        }
    }
}
