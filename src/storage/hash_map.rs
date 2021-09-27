use crate::{storage::StorageError, Storage};
use std::collections::HashMap;
use std::hash::Hash;

use std::sync::{Arc, RwLock};
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

    fn del(&self, key: K) -> Result<(), StorageError> {
        let arc = self.0.clone();
        let mut hm = arc.write().unwrap();
        hm.remove(&key);
        Ok(())
    }

    fn pop(&self, key: K) -> Result<V, StorageError> {
        let arc = self.0.clone();
        let mut hm = arc.write().unwrap();
        match hm.remove(&key) {
            Some(val) => Ok(val.clone()),
            None => Err(StorageError("key does not exists".to_string())),
        }
    }
}
