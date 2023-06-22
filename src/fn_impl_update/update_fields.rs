use types_reader::StructProperty;

use crate::{e_tag::GetETag, postgres_struct_ext::PostgresStructPropertyExt};

pub struct UpdateFields<'s> {
    items: Vec<StructProperty<'s>>,
    pub primary_key_amount: usize,
}

impl<'s> UpdateFields<'s> {
    pub fn new(items: Vec<StructProperty<'s>>) -> Self {
        let mut primary_key_amount = 0;
        for field in &items {
            if field.is_primary_key() {
                primary_key_amount += 1;
            }
        }

        Self {
            items,
            primary_key_amount,
        }
    }

    pub fn get_update_fields_amount(&self) -> usize {
        self.items.len() - self.primary_key_amount
    }

    pub fn get_fields_with_primary_key(&self) -> impl Iterator<Item = &StructProperty> {
        self.items.iter().filter(|field| field.is_primary_key())
    }

    pub fn get_fields_with_no_primary_key(&self) -> impl Iterator<Item = &StructProperty> {
        self.items.iter().filter(|field| !field.is_primary_key())
    }

    pub fn get_fields_amount(&self) -> usize {
        self.items.len()
    }
}

impl<'s> GetETag<'s> for UpdateFields<'s> {
    fn get_items(&'s self) -> &'s [StructProperty<'s>] {
        self.items.as_slice()
    }
}
