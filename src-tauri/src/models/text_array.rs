use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct TextArray<T>(pub Vec<T>);

impl<'de, T> Deserialize<'de> for TextArray<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(TextArray(Vec::<T>::deserialize(deserializer)?))
    }
}

impl<T> Serialize for TextArray<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<T> From<Vec<T>> for TextArray<T> {
    fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl<T> Into<Vec<T>> for TextArray<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}

impl<T> Deref for TextArray<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for TextArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
