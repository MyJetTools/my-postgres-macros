use types_reader::{PropertyType, StructProperty};

pub const ATTR_PRIMARY_KEY: &str = "primary_key";
pub const ATTR_DB_FIELD_NAME: &str = "db_field_name";
//pub const ATTR_IGNORE_IF_NULL: &str = "ignore_if_null";

pub const ATTR_TIMESTAMP: &str = "timestamp";
pub const ATTR_BIGINT: &str = "bigint";
pub const ATTR_JSON: &str = "json";

pub trait PostgresStructPropertyExt {
    fn is_primary_key(&self) -> bool;
    fn has_timestamp_attr(&self) -> bool;
    fn has_bigint_attr(&self) -> bool;
    fn get_db_field_name(&self) -> &str;
    fn has_json_attr(&self) -> bool;
    fn has_line_no_attr(&self) -> bool;
}

impl PostgresStructPropertyExt for StructProperty {
    fn is_primary_key(&self) -> bool {
        self.attrs.has_attr(ATTR_PRIMARY_KEY)
    }

    fn has_timestamp_attr(&self) -> bool {
        self.attrs.has_attr(ATTR_TIMESTAMP)
    }

    fn has_bigint_attr(&self) -> bool {
        self.attrs.has_attr(ATTR_BIGINT)
    }

    fn has_json_attr(&self) -> bool {
        self.attrs.has_attr(ATTR_JSON)
    }

    fn has_line_no_attr(&self) -> bool {
        self.attrs.has_attr("line_no")
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

pub fn read_value(
    result: &mut String,
    property: &StructProperty,
    sub_property: Option<&PropertyType>,
) {
    let ty = if let Some(sub_property) = sub_property {
        sub_property
    } else {
        &property.ty
    };

    if !ty.is_option() {
        if let PropertyType::Struct(_) = &ty {
            result.push_str("let sql_value = ");
        } else {
            result.push_str("let sql_value = my_postgres::code_gens::SqlValue::");
        }
    }

    match ty {
        PropertyType::U8 => {
            result.push_str("U8(");

            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }

            result.push_str(");");
        }
        PropertyType::I8 => {
            result.push_str("I8(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        PropertyType::U16 => {
            result.push_str("U16(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        PropertyType::I16 => {
            result.push_str("I16(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        PropertyType::U32 => {
            result.push_str("U32(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        PropertyType::I32 => {
            result.push_str("I32(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        PropertyType::U64 => {
            result.push_str("U64(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        PropertyType::I64 => {
            result.push_str("I64(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }

        PropertyType::F32 => {
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }

        PropertyType::F64 => {
            result.push_str("F64(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        PropertyType::USize => {
            result.push_str("USize(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        PropertyType::ISize => {
            result.push_str("ISize(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        PropertyType::String => {
            result.push_str("Str(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
                result.push_str(".as_str()");
            }
            result.push_str(");");
        }
        PropertyType::Str => {
            result.push_str("String(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        PropertyType::Bool => {
            result.push_str("Bool(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        PropertyType::DateTime => {
            if property.has_timestamp_attr() {
                result.push_str("DateTime(");
            } else {
                result.push_str("DateTimeAsUnixMicroseconds(");
            }

            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        PropertyType::OptionOf(sub_ty) => {
            read_value(result, property, Some(sub_ty.as_ref()));
        }
        PropertyType::VecOf(_) => {
            panic!("Vec not supported");
        }
        PropertyType::Struct(_) => {
            if property.has_json_attr() {
                if sub_property.is_some() {
                    result.push_str("serde_json::to_string(sql_value).unwrap();");

                    result.push_str(
                        "let sql_value = my_postgres::code_gens::SqlValue::String(sql_value);",
                    );
                } else {
                    result.push_str("serde_json::to_string(&self.");
                    result.push_str(property.name.as_str());
                    result.push_str(").unwrap();");

                    result.push_str(
                        "let sql_value = my_postgres::code_gens::SqlValue::String(sql_value);",
                    );
                }
            } else {
                if sub_property.is_some() {
                    result.push_str("sql_value.to_sql_value();");
                } else {
                    result.push_str("self.");
                    result.push_str(property.name.as_str());
                    result.push_str(".to_sql_value();");
                }
            }
        }
    }
}
