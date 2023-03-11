use types_reader::EnumCase;

const ENUM_CASE_ATTR: &str = "enum_case";

pub trait PostgresEnumExt {
    fn get_case_value(&self) -> String;
}

impl<'s> PostgresEnumExt for EnumCase<'s> {
    fn get_case_value(&self) -> String {
        match self
            .attrs
            .get_single_or_named_param(ENUM_CASE_ATTR, "value")
        {
            Ok(result) => result.as_str().to_string(),
            Err(_) => self.get_name_ident().to_string(),
        }
    }
}
