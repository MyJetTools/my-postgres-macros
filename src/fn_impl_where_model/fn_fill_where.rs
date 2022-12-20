use types_reader::{PropertyType, StructProperty};

use crate::{get_field_value::fill_sql_type, postgres_utils::PostgresStructPropertyExt};

pub fn fn_fill_where(result: &mut String, struct_properties: &[StructProperty]) {
    result.push_str("use my_postgres::SqlValueWriter;");

    result.push_str("let mut no = 0;");
    let mut no = 0;

    for struct_property in struct_properties {
        if let PropertyType::OptionOf(sub_ty) = &struct_property.ty {
            if let PropertyType::VecOf(_) = sub_ty.as_ref() {
                result.push_str("if let Some(value) = &self.");
                result.push_str(struct_property.name.as_str());
                result.push_str("{if value.len() > 0 {");
                if no > 0 {
                    fill_adding_delimiter(result);
                }
                no += 1;

                // value.len() == 1

                result.push_str("if value.len() == 1 {");
                result.push_str("sql.push_str(\"");
                result.push_str(struct_property.get_db_field_name());
                fill_op(result, struct_property);
                result.push_str("\");");

                result.push_str("value.get(0).unwrap().write(sql, params, ");
                fill_sql_type(result, struct_property);
                result.push_str(");no += 1;");

                result.push_str("} else {sql.push_str(\"");
                result.push_str(struct_property.get_db_field_name());
                result.push_str(" IN (\");for (i, itm) in value.iter().enumerate() {if i > 0 {sql.push(',');}itm.write(sql, params,");
                fill_sql_type(result, struct_property);
                result.push_str(");}sql.push(')');no += 1;}}}");
            } else {
                result.push_str("if let Some(value) = &self.");
                result.push_str(struct_property.name.as_str());
                result.push('{');
                if no > 0 {
                    fill_adding_delimiter(result);
                }
                no += 1;
                result.push_str("sql.push_str(\"");
                result.push_str(struct_property.get_db_field_name());
                fill_op(result, struct_property);
                result.push_str("\");");

                result.push_str("value.write(sql, params, ");
                fill_sql_type(result, struct_property);
                result.push_str(");no += 1;");

                result.push('}');
            }
        } else {
            if let PropertyType::VecOf(_) = &struct_property.ty {
                if no > 0 {
                    fill_adding_delimiter(result);
                }

                no += 1;

                result.push_str("sql.push_str(\"");
                result.push_str(struct_property.get_db_field_name());
                result.push_str("\");");

                fill_op(result, struct_property);

                result.push_str("self.");
                result.push_str(struct_property.name.as_str());
                result.push_str(".write(sql, params, ");
                fill_sql_type(result, struct_property);
                result.push_str(");no += 1;");
            } else {
                if no > 0 {
                    fill_adding_delimiter(result);
                }

                no += 1;

                result.push_str("sql.push_str(\"");
                result.push_str(struct_property.get_db_field_name());
                fill_op(result, struct_property);
                result.push_str("\");");

                result.push_str("self.");
                result.push_str(struct_property.name.as_str());
                result.push_str(".write(sql, params, ");
                fill_sql_type(result, struct_property);
                result.push_str(");no += 1;");
            }
        }
    }
}

fn fill_adding_delimiter(result: &mut String) {
    result.push_str("if no > 0 { sql.push_str(\" AND \"); }");
}

fn fill_op(result: &mut String, struct_propery: &StructProperty) {
    result.push_str("if self.");

    result.push_str(struct_propery.name.as_str());
    result.push_str("use_operator(){");

    if let Some(op) = struct_propery.attrs.try_get("operator") {
        result.push_str("sql.push_str(\"");
        if let Some(content) = op.content.as_ref() {
            result.push_str(extract_and_verify_operation(content));
        }
        result.push_str("\")");
    } else {
        result.push_str("sql.push_str(\"=\")");
    }

    result.push('}');
}

fn extract_and_verify_operation(src: &[u8]) -> &str {
    let result = extract_operation(src);

    if result == "="
        || result == "!="
        || result == "<"
        || result == "<="
        || result == ">"
        || result == ">="
        || result == "<>"
    {
        return result;
    }

    panic!("Invalid operator {}", result);
}

fn extract_operation(src: &[u8]) -> &str {
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

#[cfg(test)]
mod tests {

    #[test]
    fn test_1() {
        let src = "(\">\")";

        let operator = super::extract_operation(src.as_bytes());

        assert_eq!(">", operator);
    }

    #[test]
    fn test_2() {
        let src = "(>)";

        let operator = super::extract_operation(src.as_bytes());

        assert_eq!(">", operator);
    }

    #[test]
    fn test_3() {
        let src = "('>')";

        let operator = super::extract_operation(src.as_bytes());

        assert_eq!(">", operator);
    }
}
