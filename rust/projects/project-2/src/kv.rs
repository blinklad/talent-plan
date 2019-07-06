use failure::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

pub type Result<T> = std::result::Result<T, KvsError>;

#[derive(Eq, Debug, Default, PartialEq, Serialize)]
/// Container for key:value pairings, where key and value are both Strings.
/// ```rust
/// # use std::error::Error;
/// # use std::collections::HashMap;
/// # use kvs::KvStore;
/// # fn main() {
/// let mut my_kvs = KvStore::new();
/// let key1 = "key1".to_string();
/// let key2 = "key2".to_string();
/// let value1 = "value1".to_string();
///
/// my_kvs.set(key1.clone(), value1.clone());
/// assert_eq!(my_kvs.get(key1.clone()).unwrap(), value1);
/// assert_eq!(my_kvs.get(key2.clone()), None);
///
/// my_kvs.remove(key1.clone());
/// assert_eq!(my_kvs.get(key1.clone()), None);
/// #
/// # }
/// ```
pub struct KvStore {
    store: HashMap<String, String>,
}

#[derive(Eq, Fail, PartialEq, Debug)]
pub enum KvsError {
    #[fail(display = "key not found")]
    NonExistentGet,

    #[fail(display = "key not found")]
    NonExistentRemoval,
}

impl KvStore {
    /// Constructor for owned KvStore.
    pub fn new() -> Self {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// Accessor for a KvStore's, given a pairing key.
    /// Returns None if no such value is paired to the given key.
    pub fn get(&self, k: String) -> Result<Option<String>> {
        match self.store.get(&k) {
            Some(k) => Ok(Some(k.to_string())),
            None => Err(KvsError::NonExistentRemoval),
        }
    }

    /// Mutator for a KvStore's collection. Adds a value to the store and associates it with a key.
    /// Breaks the lazy initialisation and dynamically invokes memory when invoked on an newly initialised but empty KvStore.
    pub fn set(&mut self, k: String, v: String) -> Result<()> {
        //let to_bincode = |c| bincode::serialize(&c).unwrap();
        self.store.insert(k, v);
        Ok(())
    }

    /// Disassociates the underlying value of a given key. If not present, do nothing.
    pub fn remove(&mut self, k: String) -> Result<()> {
        match self.store.remove(&k) {
            Some(_) => Ok(()),
            None => Err(KvsError::NonExistentRemoval),
        }
    }

    pub fn open(_k: &Path) -> Result<Self> {
        Ok(KvStore::new())
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    /// Set and modify key:value pairings
    Set { key: String, val: String },

    /// Access stored key:value pairings
    Get { key: String },

    /// Remove stored key:value pairings
    Remove { key: String },
}
//let to_bincode = |c| bincode::serialize(&c).unwrap();
