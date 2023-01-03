pub fn render_reading_db_row_metadata_model() -> proc_macro2::TokenStream {
    quote::quote! {

        let model_field_name = if let Some(metadata) = metadata{
            if metadata.related_field_name.is_none(){
             panic!("Metadata model field_name is none");
            }
            metadata.related_field_name.unwrap()
         }
         else{
             panic!("Metadata is not defined for enum with model");
         };

         let model:String = row.get(model_field_name);
    }
}

pub fn render_sql_writing() -> proc_macro2::TokenStream {
    quote::quote! {
        let (name, model) = self.to_str();
        params.push(my_postgres::SqlValue::ValueAsStaticStr(name));
                    sql.push('$');
                    sql.push_str(params.len().to_string().as_str());

                    params.push(my_postgres::SqlValue::ValueAsString(model));
                    sql.push('$');
                    sql.push_str(params.len().to_string().as_str());
    }
}
