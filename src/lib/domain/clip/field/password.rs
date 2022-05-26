use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Password(Option<String>);

impl Password {
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();
        match password {
            Some(password) => {
                if !password.trim().is_empty() {
                    Ok(Self(Some(password)))
                } else {
                    Ok(Self(None))
                }
            },
            None => Ok(Self(None))
        }
    }

    /// Return the underlying [`String`].
    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    /// Returns whether a password has been set.
    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }

    pub fn to_str(&self) -> &str {
        match self.0 {
            Some(ref password) => password,
            None => ""
        }
    }
}

/// The Default implementation is no password.
impl Default for Password {
    fn default() -> Self {
        Self(None)
    }
}


impl FromStr for Password {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}