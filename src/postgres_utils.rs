use crate::reflection::StructProperty;

pub fn generate_field_names<'s, TIter: Iterator<Item = &'s StructProperty>>(
    result: &mut String,
    properties: TIter,
) {
    let mut first = true;
    for prop in properties {
        if !first {
            result.push(',');
        }

        result.push_str(prop.get_db_field_name());

        first = false;
    }
}

pub fn generate_field_values<'s, TIter: Iterator<Item = &'s StructProperty>>(
    result: &mut String,
    properties: TIter,
) -> i32 {
    let mut no = 1;
    let mut first = true;
    for prop in properties {
        if !first {
            result.push(',');
        }

        no = generate_set_value(result, prop, no);
        first = false;
    }

    no
}

pub fn generate_field_names_with_ignore<'s, TIter: Iterator<Item = &'s StructProperty>>(
    result: &mut String,
    properties: TIter,
) {
    for prop in properties {
        result.push_str("if !first_field{\n");
        result.push_str(" first_field = true;\n");
        result.push_str(" sql.push(',')\n");
        result.push_str("}\n");

        if prop.has_ignore_if_null_attr() {
            result.push_str("if self.");
            result.push_str(prop.name.as_str());
            result.push_str(".is_some(){\n");
        }

        result.push_str("sql.push_str(\"");
        result.push_str(prop.get_db_field_name());
        result.push_str("\");\n");

        if prop.has_ignore_if_null_attr() {
            result.push_str("}\n");
        }
    }
}

pub fn generate_field_values_with_ignore<'s, TIter: Iterator<Item = &'s StructProperty>>(
    result: &mut String,
    properties: TIter,
) {
    result.push_str("let mut no = 1;");
    for prop in properties {
        result.push_str("if !first_field{\n");
        result.push_str(" first_field = true;\n");
        result.push_str(" sql.push(',')\n");
        result.push_str("}\n");

        if prop.has_ignore_if_null_attr() {
            result.push_str("if self.");
            result.push_str(prop.name.as_str());
            result.push_str(".is_some(){\n");
        }

        if prop.ty.is_date_time() {
            result.push_str("sql.push('$');\n");
            result.push_str("sql.push_str(no.to_string().as_str());\n");
            result.push_str("no += 1;\n");
        } else {
            result.push_str("sql.push_str(\"'");

            result.push_str("sql.push_str(self.");
            result.push_str(prop.name.as_str());
            result.push_str(".to_rfc3339()");

            result.push_str("'\");\n");
        }

        if prop.has_ignore_if_null_attr() {
            result.push_str("}\n");
        }
    }
}

pub fn generate_set_value(result: &mut String, prop: &StructProperty, mut no: i32) -> i32 {
    if prop.ty.is_date_time() {
        result.push_str("'{");
        result.push_str(prop.name.as_str());
        result.push_str("}'");
    } else {
        result.push_str("$");
        result.push_str(no.to_string().as_str());

        no += 1;
    }

    no
}

pub fn generate_fields_as_params<'s, TIter: Iterator<Item = &'s StructProperty>>(
    result: &mut String,
    fields: TIter,
) {
    for prop in fields {
        if !prop.ty.is_date_time() {
            result.push_str("&self.");
            result.push_str(prop.name.as_str());
            result.push_str(",\n");
        }
    }
}

pub fn generate_date_time_reading<'s, TIter: Iterator<Item = &'s StructProperty>>(
    result: &mut String,
    fields: TIter,
) {
    for prop in fields {
        if prop.ty.is_date_time() {
            result.push(',');
            result.push_str(prop.name.as_str());
            result.push_str(" =self.");
            result.push_str(prop.name.as_str());
            result.push_str(".to_rfc3339()");
        }
    }
}

pub fn has_ignore_if_null_attributes<'s, TIter: Iterator<Item = &'s StructProperty>>(
    fields: TIter,
) -> bool {
    for prop in fields {
        if prop.has_ignore_if_null_attr() {
            return true;
        }
    }

    false
}
