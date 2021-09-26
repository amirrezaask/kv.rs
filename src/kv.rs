use std::{collections::HashMap, error::Error, fmt::Display, hash::Hash};

use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

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
}

pub struct HashMapStorage<K, V>(Arc<RwLock<HashMap<K, V>>>);

impl<K, V> HashMapStorage<K, V> {
    pub fn new() -> Self {
        HashMapStorage::<K, V>(Arc::new(RwLock::new(HashMap::new())))
    }
}
impl<K, V> Storage<K, V> for HashMapStorage<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    fn get(&self, key: K) -> Result<V, StorageError> {
        let arc = self.0.clone();
        let hm = arc.read().unwrap();

        match hm.get(&key) {
            Some(v) => Ok(v.clone()),
            None => Err(StorageError(String::from("error key does not exists"))),
        }
    }
    fn put(&self, key: K, val: V) -> Result<(), StorageError> {
        let arc = self.0.clone();
        let mut hm = arc.write().unwrap();
        hm.insert(key, val);
        Ok(())
    }
}
