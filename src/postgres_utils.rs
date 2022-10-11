use crate::reflection::PropertyType;

pub enum ReadingSoruce<'s> {
    ItSelf(&'s str),
    Variable(&'s str),
    ItSelfAsStr(&'s str),
}

impl<'s> ReadingSoruce<'s> {
    pub fn populate_reading_from(&self, result: &mut String) {
        match self {
            ReadingSoruce::ItSelf(property_name) => {
                result.push_str("self.");
                result.push_str(&property_name)
            }
            ReadingSoruce::ItSelfAsStr(property_name) => {
                result.push_str("self.");
                result.push_str(&property_name);
                result.push_str(".as_str()");
            }
            ReadingSoruce::Variable(name) => result.push_str(name),
        }
    }
}

pub fn read_value(result: &mut String, ty: &PropertyType, reading_source: ReadingSoruce) {
    result.push_str("let sql_value =  my_postgres::code_gens::SqlValue::");

    match ty {
        crate::reflection::PropertyType::U8 => {
            result.push_str("U8(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::I8 => {
            result.push_str("I8(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::U16 => {
            result.push_str("U16(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::I16 => {
            result.push_str("I16(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::U32 => {
            result.push_str("U32(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::I32 => {
            result.push_str("I32(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::U64 => {
            result.push_str("U64(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::I64 => {
            result.push_str("I64(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }

        crate::reflection::PropertyType::F32 => {
            result.push_str("F32(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }

        crate::reflection::PropertyType::F64 => {
            result.push_str("F64(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::USize => {
            result.push_str("USize(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::ISize => {
            result.push_str("ISize(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::String => {
            result.push_str("String(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::Str => {
            result.push_str("String(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::Bool => {
            result.push_str("Bool(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::DateTime => {
            result.push_str("DateTime(");
            reading_source.populate_reading_from(result);
            result.push_str(");");
        }
        crate::reflection::PropertyType::OptionOf(_) => {
            panic!("Not supported");
        }
        crate::reflection::PropertyType::VecOf(_) => {
            panic!("Vec not supported");
        }
        crate::reflection::PropertyType::Struct(_) => {
            panic!("Struct not supported");
        }
    }
}
