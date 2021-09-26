use std::{collections::HashMap, error::Error, hash::Hash};

use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct StorageError(String);
impl Error for StorageError {}

pub trait Storage<K, V> {
    fn get(&self, key: K) -> Result<V, StorageError>;
    fn put(&mut self, key: K, value: V) -> Result<(), StorageError>;
}

pub struct HashMapStorage<K, V>(Arc<RwLock<HashMap<K, V>>>);
impl<K, V> HashMapStorage<K, V> {
    fn new() -> Self {
        HashMapStorage::<K, V>(Arc::new(RwLock::new(HashMap::new())))
    }
}
impl<K, V> Storage<K, V> for HashMapStorage<K, V> {
    fn get(&self, key: K) -> Result<V, StorageError> {
        let hm: HashMap<K, V> = self.0.clone().read()?;
        match hm.get(&key) {
            Some(v) => Ok(v),
            None => Err(StorageError(String::from("key does exists"))),
        }
    }
    fn put(&mut self, key: K, val: V) -> Result<(), StorageError> {
        let mut hm: HashMap<K, V> = &self.0.clone().write()?;
        hm.insert(key, val);
        Ok(())
    }
}
