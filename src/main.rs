mod meta_schema_id;
mod program;
mod schema;

use clap::Parser;
use program::ProgramOptions;

fn main() {
    let options = ProgramOptions::parse();

    println!("{:?}", options)
}
