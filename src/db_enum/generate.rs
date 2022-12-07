use proc_macro::TokenStream;
use types_reader::EnumCase;

pub enum EnumType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
}

impl EnumType {
    pub fn as_type_name(&self) -> &str {
        match self {
            EnumType::U8 => "u8",
            EnumType::I8 => "i8",
            EnumType::U16 => "u16",
            EnumType::I16 => "i16",
            EnumType::U32 => "u32",
            EnumType::I32 => "i32",
            EnumType::U64 => "u64",
            EnumType::I64 => "i64",
        }
    }

    pub fn db_complient_type_name(&self) -> &str {
        match self {
            EnumType::U8 => "i32",
            EnumType::I8 => "i32",
            EnumType::U16 => "i32",
            EnumType::I16 => "i32",
            EnumType::U32 => "i32",
            EnumType::I32 => "i32",
            EnumType::U64 => "i64",
            EnumType::I64 => "i64",
        }
    }
}

pub fn generate(ast: &syn::DeriveInput, enum_type: EnumType) -> TokenStream {
    let name = &ast.ident.to_string();
    let enum_cases = EnumCase::read(ast);

    let mut result = String::new();

    result.push_str("impl ");
    result.push_str(name);
    result.push_str(" {");

    result.push_str("pub fn to_");
    result.push_str(enum_type.as_type_name());
    result.push_str("(&self)->");
    result.push_str(enum_type.as_type_name());

    result.push_str(" {");

    result.push_str(" match self {");

    let mut i = 0;
    for enum_case in enum_cases.as_slice() {
        result.push_str("Self::");
        result.push_str(enum_case.name.as_str());

        result.push_str(" => ");
        result.push_str(i.to_string().as_str());
        result.push(',');
        i += 1;
    }

    result.push_str("}}");

    result.push_str("pub fn as_numbered_str(&self)->&'static str {");
    super::fn_as_numbered_str::fn_as_numbered_str(&mut result, enum_cases.as_slice());
    result.push('}');

    result.push_str("pub fn from_db_value(src: ");
    result.push_str(enum_type.as_type_name());
    result.push_str(")->Self {");
    result.push_str("match src {");
    let mut i = 0;

    for enum_case in enum_cases.as_slice() {
        result.push_str(i.to_string().as_str());
        result.push_str(" => ");
        result.push_str("Self::");
        result.push_str(enum_case.name.as_str());

        result.push(',');
        i += 1;
    }

    result.push_str("_ => panic!(\"Invalid value {}\", src)");

    result.push_str("}}");

    result.push_str(
        r#"
        fn fill_select_part(sql: &mut String, field_name: &str, sql_type: Option<&str>) {
            sql.push_str(field_name);
        }
    "#,
    );

    result.push('}');

    result.push_str("impl<'s> my_postgres::SqlValueWriter<'s> for ");
    result.push_str(name);

    result.push_str(
        r#"{
        fn write(
            &'s self,
            sql: &mut String,
            params: &mut Vec<&'s (dyn tokio_postgres::types::ToSql + Sync)>,
            sql_type: Option<&'static str>,
        ) {
            sql.push_str(self.as_numbered_str());
        }
    }
    "#,
    );

    super::fn_from_db_row::fn_from_db_row(&mut result, name, &enum_type);
    result.parse().unwrap()
}
