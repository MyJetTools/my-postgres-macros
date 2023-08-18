use types_reader::StructProperty;

use crate::e_tag::GetETag;

pub struct InsertFields<'s> {
    items: Vec<StructProperty<'s>>,
}

impl<'s> InsertFields<'s> {
    pub fn new(items: Vec<StructProperty<'s>>) -> Self {
        Self { items }
    }

    pub fn get_fields_amount(&self) -> usize {
        self.items.len()
    }

    pub fn as_slice(&'s self) -> &'s [StructProperty<'s>] {
        self.items.as_slice()
    }
}

impl<'s> GetETag<'s> for InsertFields<'s> {
    fn get_items(&'s self) -> Vec<&'s StructProperty<'s>> {
        let mut result = Vec::with_capacity(self.items.len());
        for field in &self.items {
            result.push(field)
        }
        result
    }
}
