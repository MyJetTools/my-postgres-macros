use types_reader::EnumCase;

const ENUM_CASE_ATTR: &str = "enum_case";

pub trait PostgresEnumExt {
    fn get_case_value(&self) -> Result<String, syn::Error>;
}

impl<'s> PostgresEnumExt for EnumCase<'s> {
    fn get_case_value(&self) -> Result<String, syn::Error> {
        let result = match self
            .attrs
            .get_single_or_named_param(ENUM_CASE_ATTR, "value")
        {
            Ok(result) => result.get_any_value_as_string()?.to_string(),
            Err(_) => self.get_name_ident().to_string(),
        };

        Ok(result)
    }
}
