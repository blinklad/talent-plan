// TODO k : v inconsistency
#[allow(unused_imports)]
use failure::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, KvsError>;
static STORE: &'static str = "store.bc";

#[derive(Debug)] // TODO
                 // Container for key:value pairings, where key and value are both Strings.
                 // ```rust
                 // # use std::error::Error;
                 // # use std::collections::HashMap;
                 // # use kvs::KvStore;
                 // # fn main() {
                 // let mut my_kvs = KvStore::new();
                 // let key1 = "key1".to_string();
                 // let key2 = "key2".to_string();
                 // let value1 = "value1".to_string();
                 //
                 // my_kvs.set(key1.clone(), value1.clone());
                 // assert_eq!(my_kvs.get(key1.clone()).unwrap(), value1);
                 // assert_eq!(my_kvs.get(key2.clone()), None);
                 //
                 // my_kvs.remove(key1.clone());
                 // assert_eq!(my_kvs.get(key1.clone()), None);
                 // #
                 // # }
                 // ```
pub struct KvStore {
    store: HashMap<String, String>,
    reader: BufReader<File>,
    writer: BufWriter<File>,
    path: PathBuf,
}

impl KvStore {
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
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let to_bincode = |cmd: Command| bincode::serialize(&cmd).unwrap();

        let cmd = Command::Set {
            key: &key,
            value: &value,
        };

        self.writer.write(&to_bincode(cmd))?;
        Ok(())
    }

    /// Disassociates the underlying value of a given key. If not present, do nothing.
    pub fn remove(&mut self, k: String) -> Result<()> {
        match self.store.remove(&k) {
            Some(_) => Ok(()),
            None => Err(KvsError::NonExistentRemoval),
        }
    }

    pub fn open<P: PathBuf::FromStr>(dirpath: P) -> Result<Self> {
        let mut path = PathBuf::from(dirpath);
        path.push(STORE);

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let writer = BufWriter::new(file);
        let store = HashMap::new();

        Ok(KvStore {
            store,
            reader,
            writer,
            path: path.to_owned(),
        })
    }
}

#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "file not found")]
    IoError(std::io::Error),

    #[fail(display = "Key not found")]
    NonExistentRemoval,

    #[fail(display = "Key not found")]
    NonExistentGet,
}

impl From<std::io::Error> for KvsError {
    fn from(error: std::io::Error) -> Self {
        KvsError::IoError(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Command<'a> {
    /// Set and modify key:value pairings
    Set { key: &'a str, value: &'a str },

    /// Access stored key:value pairings
    Get { key: &'a str },

    /// Remove stored key:value pairings
    Remove { key: &'a str },
}
//let to_bincode = |c| bincode::serialize(&c).unwrap();
