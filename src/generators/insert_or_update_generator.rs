use super::{NumberedParams, SqlLineBuilder};

pub struct InsertOrUpdateGenerator {
    insert_fields: SqlLineBuilder,
    insert_values: SqlLineBuilder,
    update_fields: SqlLineBuilder,
    update_values: SqlLineBuilder,
    numbered_params: NumberedParams,
}

impl InsertOrUpdateGenerator {
    pub fn new() -> Self {
        Self {
            insert_fields: SqlLineBuilder::new(",".to_string()),
            insert_values: SqlLineBuilder::new(",".to_string()),
            update_fields: SqlLineBuilder::new(",".to_string()),
            update_values: SqlLineBuilder::new(",".to_string()),
            numbered_params: NumberedParams::new(),
        }
    }

    pub fn add_insert_field_value(&mut self, field_name: &str, value: &str) {
        self.insert_fields.add(field_name);
        let no = self.numbered_params.add_or_get(field_name);
        self.insert_values.add(&format!("${}", no));
    }

    pub fn add_insert_field_with_raw_value(&mut self, field_name: &str, value: &str) {
        self.insert_fields.add(field_name);
        let no = self.numbered_params.add_or_get(field_name);
        self.insert_values.add(value);
    }

    pub fn add_update_field_value(&mut self, field_name: &str, value: &str) {
        self.insert_fields.add(field_name);
        let no = self.numbered_params.add_or_get(field_name);
        self.insert_values.add(&format!("${}", no));
    }

    pub fn add_update_field_with_raw_value(&mut self, field_name: &str, value: &str) {
        self.insert_fields.add(field_name);
        let no = self.numbered_params.add_or_get(field_name);
        self.insert_values.add(value);
    }

    pub fn generate_sql(&self, result: &mut String, table_name: &str, pk_name: &str) {
        result.push_str("INSERT INTO ");
        result.push_str(table_name);
        result.push_str(" (");
        result.push_str(self.insert_fields.as_str());
        result.push_str(") VALUES (");
        result.push_str(self.insert_values.as_str());
        result.push_str(") ON CONFLICT ON CONSTRAINT ");
        result.push_str(pk_name);
        result.push_str(" DO UPDATE SET (");
        result.push_str(self.update_fields.as_str());
        result.push_str(") = (");
        result.push_str(self.update_values.as_str());
        result.push_str(")");
    }

    pub fn get_numbred_fields<'s>(&'s self) -> impl Iterator<Item = &'s String> {
        self.numbered_params.params.iter().map(|s| s)
    }
}
