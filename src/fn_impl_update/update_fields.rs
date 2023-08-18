use std::collections::HashMap;

use types_reader::StructProperty;

use crate::{e_tag::GetETag, postgres_struct_ext::PostgresStructPropertyExt};

pub struct UpdateFields<'s> {
    update_fields: Vec<&'s StructProperty<'s>>,
    where_fields: Vec<&'s StructProperty<'s>>,
}

impl<'s> UpdateFields<'s> {
    pub fn new_from_update_model(items: &'s [StructProperty<'s>]) -> Self {
        let mut update_fields = Vec::with_capacity(items.len());
        let mut where_fields = Vec::with_capacity(items.len());

        for field in items {
            if field.is_primary_key() {
                where_fields.push(field)
            } else {
                update_fields.push(field)
            }
        }

        Self {
            update_fields,
            where_fields,
        }
    }

    pub fn new_from_table_schema(
        fields: &'s [&'s StructProperty<'s>],
    ) -> Result<HashMap<String, Self>, syn::Error> {
        let mut hash_map = HashMap::new();

        for prop in fields {
            if let Some(generate_update_models) = prop.get_generate_additional_update_models()? {
                for generate_update_model in generate_update_models {
                    if !hash_map.contains_key(generate_update_model.struct_name.as_str()) {
                        hash_map.insert(generate_update_model.struct_name.to_string(), Vec::new());
                    }

                    hash_map
                        .get_mut(generate_update_model.struct_name.as_str())
                        .unwrap()
                        .push((generate_update_model, prop));
                }
            }
        }

        let mut result = HashMap::new();

        for (struct_name, model_fields) in hash_map {
            let mut update_fields = Vec::new();
            let mut where_fields = Vec::new();

            for model_field in model_fields {
                if model_field.0.is_where {
                    where_fields.push(*model_field.1);
                } else {
                    update_fields.push(*model_field.1);
                }
            }

            result.insert(struct_name, Self::new(update_fields, where_fields));
        }

        Ok(result)
    }

    pub fn new(
        update_fields: Vec<&'s StructProperty<'s>>,
        where_fields: Vec<&'s StructProperty<'s>>,
    ) -> Self {
        Self {
            update_fields,
            where_fields,
        }
    }

    pub fn get_update_fields(&'s self) -> &'s [&'s StructProperty<'s>] {
        &self.update_fields
    }

    pub fn get_where_fields(&'s self) -> &'s [&'s StructProperty<'s>] {
        self.where_fields.as_slice()
    }

    pub fn get_fields_amount(&self) -> usize {
        self.update_fields.len() + self.where_fields.len()
    }
}

impl<'s> GetETag<'s> for UpdateFields<'s> {
    fn get_items(&'s self) -> Vec<&'s StructProperty<'s>> {
        let mut result = Vec::with_capacity(self.update_fields.len() + self.where_fields.len());

        for field in &self.update_fields {
            result.push(*field)
        }

        for field in &self.where_fields {
            result.push(*field)
        }
        result
    }
}
