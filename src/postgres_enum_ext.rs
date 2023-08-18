use types_reader::EnumCase;

const ENUM_CASE_ATTR: &str = "enum_case";

pub trait PostgresEnumExt {
    fn get_case_string_value(&self) -> Result<String, syn::Error>;
    fn get_case_number_value(&self) -> Result<Option<i64>, syn::Error>;
    fn get_case_any_string_value(&self) -> Result<String, syn::Error>;
}

impl<'s> PostgresEnumExt for EnumCase<'s> {
    fn get_case_string_value(&self) -> Result<String, syn::Error> {
        let result = match self
            .attrs
            .get_single_or_named_param(ENUM_CASE_ATTR, "value")
        {
            Ok(result) => result.unwrap_as_string_value()?.as_str().to_string(),
            Err(_) => self.get_name_ident().to_string(),
        };

        Ok(result)
    }

    fn get_case_any_string_value(&self) -> Result<String, syn::Error> {
        let result = match self
            .attrs
            .get_single_or_named_param(ENUM_CASE_ATTR, "value")
        {
            Ok(result) => result.get_any_value_as_str()?.to_string(),
            Err(_) => self.get_name_ident().to_string(),
        };

        Ok(result)
    }

    fn get_case_number_value(&self) -> Result<Option<i64>, syn::Error> {
        let result = match self
            .attrs
            .try_get_single_or_named_params(ENUM_CASE_ATTR, ["value", "id"].into_iter())
        {
            Some(result) => Some(result.unwrap_as_number_value()?.as_i64()),
            None => None,
        };

        Ok(result)
    }
}
