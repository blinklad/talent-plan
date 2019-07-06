#[deny(missing_docs)]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate serde;
extern crate structopt;

pub use kv::{KvStore, Result};

mod kv;
