use macros_utils::ParamValue;
use types_reader::{PropertyType, StructProperty};

pub const ATTR_PRIMARY_KEY: &str = "primary_key";
pub const ATTR_DB_FIELD_NAME: &str = "db_field_name";
//pub const ATTR_IGNORE_IF_NULL: &str = "ignore_if_null";

pub const ATTR_SQL_TYPE: &str = "sql_type";
pub const ATTR_JSON: &str = "json";

pub trait PostgresStructPropertyExt {
    fn is_primary_key(&self) -> bool;

    fn get_sql_type(&self) -> Option<ParamValue>;

    fn get_db_field_name(&self) -> Result<String, syn::Error>;
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
        self.attrs.contains_key(ATTR_PRIMARY_KEY)
    }

    fn has_ignore_attr(&self) -> bool {
        self.attrs.contains_key("ignore")
    }

    fn get_sql_type(&self) -> Option<ParamValue> {
        let attr = self.attrs.get(ATTR_SQL_TYPE)?;

        match attr {
            Some(result) => result.get_from_single_or_named("name"),
            None => None,
        }
    }

    fn has_ignore_if_null_attr(&self) -> bool {
        self.attrs.contains_key("ignore_if_null")
    }

    fn has_json_attr(&self) -> bool {
        self.attrs.contains_key(ATTR_JSON)
    }

    fn is_line_no(&self) -> bool {
        self.attrs.contains_key("line_no") || self.name == "line_no"
    }

    fn get_db_field_name(&self) -> Result<String, syn::Error> {
        if let Some(attr) = self.attrs.get(ATTR_DB_FIELD_NAME) {
            match attr {
                Some(result) => match result.get_from_single_or_named("name") {
                    Some(result) => return Ok(result.get_value_as_str().to_string()),
                    None => {
                        return Err(syn::Error::new_spanned(
                            self.field,
                            format!(
                                "Attribute {} should have name inside ()",
                                ATTR_DB_FIELD_NAME
                            ),
                        ))
                    }
                },
                None => {
                    return Err(syn::Error::new_spanned(
                        self.field,
                        format!("Attribute db_field_name should not be empty"),
                    ))
                }
            }
        }

        Ok(self.name.clone())
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
            if let Some(attr) = itm.get_sql_type() {
                let attr = attr.get_value_as_str();
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
