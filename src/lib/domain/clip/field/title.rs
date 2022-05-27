use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use rocket::form::{self, DataField, FromFormField, ValueField};

/// The title field for a [`Clip`](crate::domain::clip::Clip).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Title(Option<String>);

impl Title{
    /// Create a new `Title` field.
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        let title: Option<String> = title.into();
        match title {
            Some(title) => {
                if !title.trim().is_empty() {
                    Self(Some(title))
                } else {
                    Self(None)
                }
            }
            None => Self(None)
        }
    }

    /// Return the underlying [`Option<String>`](`String`).
    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

/// The Default implementation is no title.
impl Default for Title {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Title {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value.to_owned()))
    }
}