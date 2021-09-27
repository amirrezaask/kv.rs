use std::{collections::HashMap, error::Error, fmt::Display, hash::Hash};

use std::sync::{Arc, RwLock};
mod hash_map;

pub use hash_map::HashMapStorage;

#[derive(Debug)]
pub struct StorageError(String);
impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "storage layer error: {}", &self.0)
    }
}
impl Error for StorageError {}

pub trait Storage<K, V> {
    fn get(&self, key: K) -> Result<V, StorageError>;
    fn put(&self, key: K, value: V) -> Result<(), StorageError>;
    fn del(&self, key: K) -> Result<(), StorageError>;
    fn pop(&self, key: K) -> Result<V, StorageError>;
}
