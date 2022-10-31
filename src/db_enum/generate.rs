use proc_macro::TokenStream;
use types_reader::EnumCase;

pub fn generate(ast: &syn::DeriveInput, type_name: &str) -> TokenStream {
    let name = &ast.ident.to_string();
    let enum_cases = EnumCase::read(ast);

    let mut result = String::new();

    result.push_str("impl ");
    result.push_str(name);
    result.push_str(" {");

    result.push_str("fn to_db_value(&self)->");

    result.push_str(type_name);

    result.push_str(" {");

    result.push_str(" match self {");

    let mut i = 0;
    for enum_case in enum_cases.as_slice() {
        result.push_str(enum_case.name.as_str());

        result.push_str(" => ");

        result.push_str(i.to_string().as_str());
        result.push(',');
        i += 1;
    }

    result.push_str("}}}");

    result.parse().unwrap()
}
