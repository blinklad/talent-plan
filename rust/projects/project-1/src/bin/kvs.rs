extern crate structopt;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "kvs", about = "Key value storage")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
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

fn main() {
    let app = Cli::from_args();

    match app.cmd {
        set => {
            eprintln!("unimplemented");
            panic!()
        }
        get => {
            eprintln!("unimplemented");
            panic!();
        }
        remove => {
            eprintln!("unimplemented");
            panic!();
        }
    };
}
