use types_reader::StructProperty;

use quote::quote;

pub fn fn_fill_where(struct_properties: &[StructProperty]) -> proc_macro2::TokenStream {
    let mut no = 0;

    let mut lines: Vec<proc_macro2::TokenStream> = Vec::new();

    for struct_property in struct_properties {
        if no > 0 {
            lines.push(quote! {
                sql.push_str(" AND ");
            });
        }
        let name = struct_property.name.as_str();
        let prop_name_ident = struct_property.name_ident;
        let sql_type = crate::get_field_value::fill_sql_type(struct_property);

        let op = fill_op(struct_property);

        lines.push(quote! {
            sql.push_str(#name);
            #op
            self.#prop_name_ident.write(sql, params, #sql_type);
        });

        no += 1;
    }

    quote! {
        use my_postgres::SqlValueWriter;
        #(#lines)*
    }
}

fn fill_op(struct_property: &StructProperty) -> proc_macro2::TokenStream {
    let prop_name_ident = struct_property.name_ident;
    //sql.push_str(self.#prop_name_ident.get_default_operator());

    if let Some(op) = struct_property.attrs.try_get("operator") {
        if let Some(content) = op.content.as_ref() {
            let op = extract_and_verify_operation(content);
            return quote! {
                sql.push_str(#op);
            }
            .into();
        } else {
            return quote! {
                sql.push_str(self.#prop_name_ident.get_default_operator());
            }
            .into();
        }
    } else {
        return quote! {
            sql.push_str(self.#prop_name_ident.get_default_operator());
        }
        .into();
    }
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
