use std::borrow::{Borrow, Cow};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct Username<'a>(Cow<'a, str>);

impl<'a> Username<'a> {
    pub fn as_str(&'a self) -> &'a str {
        self.0.borrow()
    }
    pub fn to_string(&'a self) -> String {
        self.0.to_string()
    }
    pub fn to_static(&self) -> Username<'static> {
        self.as_str().to_owned().into()
    }
}

impl<'a, T> From<T> for Username<'a>
    where T: Into<Cow<'a, str>>
{
    fn from(value: T) -> Username<'a> {
        Username(value.into())
    }
}