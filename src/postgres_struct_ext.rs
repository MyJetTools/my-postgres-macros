use types_reader::{attribute_params::ParamValue, PropertyType, StructProperty};

pub const ATTR_PRIMARY_KEY: &str = "primary_key";
pub const ATTR_DB_FIELD_NAME: &str = "db_field_name";
//pub const ATTR_IGNORE_IF_NULL: &str = "ignore_if_null";

pub const ATTR_SQL_TYPE: &str = "sql_type";
pub const ATTR_JSON: &str = "json";

pub trait PostgresStructPropertyExt {
    fn is_primary_key(&self) -> bool;

    fn get_sql_type(&self) -> Result<ParamValue, syn::Error>;

    fn get_db_field_names(&self) -> proc_macro2::TokenStream;

    fn get_db_field_name_as_string(&self) -> String;
    fn has_json_attr(&self) -> bool;

    fn has_ignore_attr(&self) -> bool;
    fn has_ignore_if_null_attr(&self) -> bool;

    fn is_line_no(&self) -> bool;

    fn sql_value_to_mask(&self) -> bool;
}

impl<'s> PostgresStructPropertyExt for StructProperty<'s> {
    fn sql_value_to_mask(&self) -> bool {
        if self.ty.is_string() {
            return true;
        }

        if let PropertyType::OptionOf(sub_ty) = &self.ty {
            if sub_ty.is_string() {
                return true;
            }
        }

        false
    }

    fn is_primary_key(&self) -> bool {
        self.attrs.has_attr(ATTR_PRIMARY_KEY)
    }

    fn has_ignore_attr(&self) -> bool {
        self.attrs.has_attr("ignore")
    }

    fn get_sql_type(&self) -> Result<ParamValue, syn::Error> {
        self.attrs.get_single_or_named_param(ATTR_SQL_TYPE, "name")
    }

    fn has_ignore_if_null_attr(&self) -> bool {
        self.attrs.has_attr("ignore_if_null")
    }

    fn has_json_attr(&self) -> bool {
        self.attrs.has_attr(ATTR_JSON)
    }

    fn is_line_no(&self) -> bool {
        self.attrs.has_attr("line_no") || self.name == "line_no"
    }

    fn get_db_field_names(&self) -> proc_macro2::TokenStream {
        if let Ok(attr) = self.attrs.get_attr(ATTR_DB_FIELD_NAME) {
            let field_1 = if let Ok(result) = attr.get_from_single_or_named("name") {
                result.as_str().to_string()
            } else {
                self.name.to_string()
            };

            if let Ok(result) = attr.get_from_single_or_named("model_field_name") {
                let field_2 = result.as_str().to_string();
                return quote::quote!(&[#field_1, #field_2]);
            }

            return quote::quote!(&[#field_1]);
        }

        let field_1 = self.name.to_string();

        quote::quote!(&[#field_1])
    }

    fn get_db_field_name_as_string(&self) -> String {
        if let Ok(attr) = self
            .attrs
            .get_single_or_named_param(ATTR_DB_FIELD_NAME, "model_field_name")
        {
            return attr.as_str().to_string();
        }

        return self.name.to_string();
    }
}

pub fn filter_fields(
    src: Vec<StructProperty>,
) -> Result<Vec<StructProperty>, proc_macro::TokenStream> {
    let mut result = Vec::with_capacity(src.len());

    for itm in src {
        if itm.has_ignore_attr() {
            continue;
        }

        if itm.ty.is_date_time() {
            if let Ok(attr) = itm.get_sql_type() {
                let attr = attr.as_str();
                if attr != "timestamp" && attr != "bigint" {
                    let result = syn::Error::new_spanned(
                        itm.field,
                        format!("Sql type must be 'timestamp' or 'bigint'"),
                    );

                    let err = result.to_compile_error();
                    return Err(quote::quote!(#err).into());
                }
            } else {
                let result = syn::Error::new_spanned(
                    itm.field,
                    format!("Please specify sql_type for {}", itm.name),
                );

                let err = result.to_compile_error();
                return Err(quote::quote!(#err).into());
            }
        }

        result.push(itm);
    }

    return Ok(result);
}
