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
    pub fn as_sql_value_name(&self) -> &str {
        match self {
            EnumType::U8 => "U8",
            EnumType::I8 => "I8",
            EnumType::U16 => "U16",
            EnumType::I16 => "I16",
            EnumType::U32 => "U32",
            EnumType::I32 => "I32",
            EnumType::U64 => "U64",
            EnumType::I64 => "I64",
        }
    }

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
}

pub fn generate(ast: &syn::DeriveInput, type_name: EnumType) -> TokenStream {
    let name = &ast.ident.to_string();
    let enum_cases = EnumCase::read(ast);

    let mut result = String::new();

    result.push_str("impl ");
    result.push_str(name);
    result.push_str(" {");

    result.push_str("pub fn to_sql_value(&self)->my_postgres::SqlValue");

    result.push_str(" {");

    result.push_str(" match self {");

    let mut i = 0;
    for enum_case in enum_cases.as_slice() {
        result.push_str(enum_case.name.as_str());

        result.push_str(" => ");

        result.push_str("my_postgres::SqlValue::");

        result.push_str(type_name.as_sql_value_name());

        result.push_str("(");

        result.push_str(i.to_string().as_str());
        result.push_str(")");
        result.push(',');
        i += 1;
    }

    result.push_str("}}");

    result.push_str("pub fn to_");
    result.push_str(type_name.as_type_name());
    result.push_str("(&self)->");
    result.push_str(type_name.as_type_name());

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

    result.push_str("}}");

    result.push_str("pub fn as_numbered_str(&self)->&'static str {");

    result.push_str(" match self {");

    let mut i = 0;
    for enum_case in enum_cases.as_slice() {
        result.push_str(enum_case.name.as_str());

        result.push_str(" => \"");
        result.push_str(i.to_string().as_str());
        result.push('"');
        result.push(',');
        i += 1;
    }

    result.push_str("}}");

    result.push_str("pub fn from_db_value(src: ");
    result.push_str(type_name.as_type_name());
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

    result.push_str("}}}");

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

    result.parse().unwrap()
}
