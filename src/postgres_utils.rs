use proc_macro::TokenStream;
use types_reader::{PropertyType, StructProperty};

pub const ATTR_PRIMARY_KEY: &str = "primary_key";
pub const ATTR_DB_FIELD_NAME: &str = "db_field_name";
//pub const ATTR_IGNORE_IF_NULL: &str = "ignore_if_null";

pub const ATTR_SQL_TYPE: &str = "sql_type";
pub const ATTR_JSON: &str = "json";

pub trait PostgresStructPropertyExt {
    fn is_primary_key(&self) -> bool;

    fn has_sql_type_attr(&self) -> bool;
    fn get_sql_type(&self) -> Option<String>;

    fn get_db_field_name(&self) -> &str;
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

    fn has_sql_type_attr(&self) -> bool {
        self.attrs.has_attr(ATTR_SQL_TYPE)
    }
    fn get_sql_type(&self) -> Option<String> {
        let attr = self.attrs.try_get(ATTR_SQL_TYPE)?;

        if let Some(value) = &attr.content {
            return crate::postgres_utils::extract_attr_value(value)
                .to_string()
                .into();
        }
        panic!("Attribute {} has to have value inside ()", ATTR_SQL_TYPE);
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

    fn get_db_field_name(&self) -> &str {
        if let Some(attr) = self.attrs.try_get(ATTR_DB_FIELD_NAME) {
            match attr.get_as_string("name") {
                Some(result) => return result,
                None => panic!("Attribute db_field_name must have a name"),
            }
        }

        self.name.as_str()
    }
}

pub fn filter_fields(src: Vec<StructProperty>) -> Result<Vec<StructProperty>, TokenStream> {
    let mut result = Vec::with_capacity(src.len());

    for itm in src {
        if itm.has_ignore_attr() {
            continue;
        }

        if itm.ty.is_date_time() {
            if !itm.has_sql_type_attr() {
                let result = syn::Error::new_spanned(
                    itm.get_field_name_ident(),
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

pub fn extract_attr_value(src: &[u8]) -> &str {
    let src = &src[1..src.len() - 1];

    for i in 0..src.len() {
        if src[i] == b'"' || src[i] == b'\'' {
            let b = src[i];

            for j in 1..src.len() {
                let pos = src.len() - j;

                if src[pos] == b {
                    let result = &src[i + 1..pos];

                    let result = std::str::from_utf8(result).unwrap();
                    return result;
                }
            }
        }
    }

    std::str::from_utf8(src).unwrap()
}
