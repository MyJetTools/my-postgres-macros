use types_reader::{attribute_params::ParamValue, PropertyType, StructProperty};

pub const ATTR_PRIMARY_KEY: &str = "primary_key";
pub const ATTR_DB_FIELD_NAME: &str = "db_field_name";
//pub const ATTR_IGNORE_IF_NULL: &str = "ignore_if_null";

pub const ATTR_SQL_TYPE: &str = "sql_type";
pub const ATTR_JSON: &str = "json";

pub struct IndexAttr {
    pub id: u8,
    pub index_name: String,
    pub name: String,
    pub is_unique: bool,
    pub order: String,
}

pub trait PostgresStructPropertyExt {
    fn is_primary_key(&self) -> bool;

    fn get_primary_key_id(&self) -> Result<Option<u8>, syn::Error>;

    fn get_sql_type(&self) -> Result<ParamValue, syn::Error>;

    fn get_db_field_name_as_token(&self) -> proc_macro2::TokenStream;

    fn get_db_field_name_as_string(&self) -> String;

    fn get_model_db_field_name_as_string(&self) -> Option<ParamValue>;

    fn has_json_attr(&self) -> bool;

    fn has_ignore_attr(&self) -> bool;
    fn has_ignore_if_null_attr(&self) -> bool;

    fn is_line_no(&self) -> bool;

    fn sql_value_to_mask(&self) -> bool;

    fn get_field_metadata(&self) -> proc_macro2::TokenStream;

    fn has_ignore_table_column(&self) -> bool;

    fn get_index_attrs(&self) -> Result<Option<Vec<IndexAttr>>, syn::Error>;
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

    fn get_primary_key_id(&self) -> Result<Option<u8>, syn::Error> {
        if let Some(value) = self.attrs.try_get_attr(ATTR_PRIMARY_KEY) {
            let value = value.get_single_param()?;
            return Ok(Some(value.get_value("must be value from 0..255".into())?));
        }

        Ok(None)
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

    fn has_ignore_table_column(&self) -> bool {
        self.attrs.has_attr("ignore_table_column")
    }

    fn has_json_attr(&self) -> bool {
        self.attrs.has_attr(ATTR_JSON)
    }

    fn is_line_no(&self) -> bool {
        self.attrs.has_attr("line_no") || self.name == "line_no"
    }

    fn get_db_field_name_as_token(&self) -> proc_macro2::TokenStream {
        if let Ok(attr) = self.attrs.get_attr(ATTR_DB_FIELD_NAME) {
            if let Ok(result) = attr.get_from_single_or_named("name") {
                let name = result.as_str().to_string();
                return quote::quote!(#name);
            }
        }

        let name = self.name.as_str();

        quote::quote!(#name)
    }

    fn get_db_field_name_as_string(&self) -> String {
        if let Ok(attr) = self
            .attrs
            .get_single_or_named_param(ATTR_DB_FIELD_NAME, "name")
        {
            return attr.as_str().to_string();
        }

        return self.name.to_string();
    }

    fn get_model_db_field_name_as_string(&self) -> Option<ParamValue> {
        if let Ok(attr) = self.attrs.get_named_param(ATTR_DB_FIELD_NAME, "name") {
            return Some(attr);
        }

        return None;
    }

    fn get_index_attrs(&self) -> Result<Option<Vec<IndexAttr>>, syn::Error> {
        let attrs = self.attrs.try_get_attrs("db_index");

        if attrs.is_none() {
            return Ok(None);
        }

        let attrs = attrs.unwrap();

        let mut result = Vec::with_capacity(attrs.len());

        for attr in attrs {
            let id = attr
                .get_named_param("id")?
                .get_value("id must be a number 0..255".into())?;

            let index_name = attr.get_named_param("index_name")?.as_str().to_string();
            let is_unique = attr.get_named_param("is_unique")?.get_bool_value()?;

            let order = attr.get_named_param("order")?.as_str().to_string();
            if order != "DESC" && order != "ASC" {
                return Err(syn::Error::new_spanned(
                    attr.get_attr_token(),
                    "order must be DESC or ASC",
                ));
            }

            result.push(IndexAttr {
                id,
                index_name,
                is_unique,
                order,
                name: self.get_db_field_name_as_string(),
            })
        }

        Ok(Some(result))
    }

    fn get_field_metadata(&self) -> proc_macro2::TokenStream {
        let model_field_name = self.get_model_db_field_name_as_string();

        let sql_type = self.get_sql_type();

        if model_field_name.is_none() && sql_type.is_err() {
            return quote::quote!(None);
        }

        let model_field_name = if let Some(model_field_name) = model_field_name {
            let model_field_name = model_field_name.as_str();
            quote::quote!(Some(#model_field_name))
        } else {
            quote::quote!(None)
        };

        let sql_type = if let Ok(sql_type) = sql_type {
            let sql_type = sql_type.as_str();
            quote::quote!(Some(#sql_type))
        } else {
            quote::quote!(None)
        };

        quote::quote!({
            Some(my_postgres::SqlValueMetadata{
                sql_type: #sql_type,
                related_field_name: #model_field_name
            })
        })
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

        let is_date_time = match &itm.ty {
            PropertyType::DateTime => true,
            PropertyType::OptionOf(sub_ty) => sub_ty.is_date_time(),
            _ => false,
        };

        if is_date_time {
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
