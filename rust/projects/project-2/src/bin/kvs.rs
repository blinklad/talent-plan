use kvs::*;
use std::process;
use structopt::StructOpt;

const IO_ERR: i32 = 74;

#[derive(Debug, StructOpt)]
#[structopt(name = "kvs", about = "Key value storage")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "set")]
    /// Set and modify key:value pairings
    Set { key: String, val: String },

    #[structopt(name = "get")]
    /// Access stored key:value pairings
    Get { key: String },

    #[structopt(name = "rm")]
    /// Remove stored key:value pairings
    Remove { key: String },
}

fn main() -> Result<()> {
    let mut store = kvs::KvStore::open("~/");

    match Cli::from_args().cmd {
        Command::Get { key } => match store.get(key) {
            Ok(_) => Ok(()),
            Err(_) => {
                process::exit(IO_ERR);
            }
        },
        Command::Set { key, val } => match store.set(key, val) {
            Ok(_) => Ok(()),
            Err(_) => {
                process::exit(IO_ERR);
            }
        },
        Command::Remove { key } => match store.remove(key) {
            Ok(_) => Ok(()),
            Err(_) => {
                process::exit(IO_ERR);
            }
        },
    }
}
