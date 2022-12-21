use super::{utils, BinanceSigner};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecretKey(String);

impl<T> From<T> for SecretKey
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        Self(String::from(value))
    }
}

impl Deref for SecretKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SecretKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BinanceSigner for SecretKey {
    fn get_signature_from_query(&self, query: &str) -> String {
        utils::signature_from_query(query, self.as_bytes())
    }
}
