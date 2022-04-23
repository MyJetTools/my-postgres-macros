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

        if prop.ty.is_date_time() {
            result.push_str("'{");
            result.push_str(prop.name.as_str());
            result.push_str("}'");
        } else {
            result.push_str("$");
            result.push_str(no.to_string().as_str());

            no += 1;
        }

        first = false;
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
