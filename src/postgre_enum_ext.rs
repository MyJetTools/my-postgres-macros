use types_reader::{attribute_params::ParamValue, EnumCase};

const ENUM_CASE_ATTR: &str = "enum_case";

pub trait PostgresEnumExt {
    fn get_case_name(&self) -> Result<ParamValue, syn::Error>;
}

impl<'s> PostgresEnumExt for EnumCase<'s> {
    fn get_case_name(&self) -> Result<ParamValue, syn::Error> {
        self.attrs
            .get_single_or_named_param(ENUM_CASE_ATTR, "value")
    }
}
