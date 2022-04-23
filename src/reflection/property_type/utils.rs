use super::PropertyType;

pub fn get_generic(type_path: &syn::TypePath) -> PropertyType {
    for path in &type_path.path.segments {
        if let syn::PathArguments::AngleBracketed(args) = &path.arguments {
            for arg in &args.args {
                if let syn::GenericArgument::Type(ty) = &arg {
                    if let syn::Type::Path(tp) = ty {
                        for path in &tp.path.segments {
                            return PropertyType::parse(path.ident.to_string().as_str(), tp);
                        }
                    }
                }
            }
        }
    }

    panic!("Can not get generic from the type {:?}", type_path);
}

pub fn simple_type_to_string(field: &syn::TypePath) -> String {
    let mut result = None;
    for segment in &field.path.segments {
        result = Some(segment);
    }

    let result = result.unwrap();

    result.ident.to_string()
}
