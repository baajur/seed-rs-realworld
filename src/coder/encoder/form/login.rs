use crate::entity::form::{login::ValidForm as EntityValidForm, FormField};
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct ValidForm<'a> {
    user: IndexMap<&'a str, &'a str>,
}

impl<'a> ValidForm<'a> {
    pub fn new(form: &'a EntityValidForm) -> Self {
        ValidForm {
            user: form
                .iter_keys_and_fields()
                .map(|(key, field)| (*key, field.value()))
                .collect(),
        }
    }
}
