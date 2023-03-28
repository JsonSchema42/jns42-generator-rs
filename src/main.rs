mod meta_schema_id;
mod programs;
mod schemas;

use clap::Parser;
use programs::{run_program, ProgramOptions};

fn main() {
    let options = ProgramOptions::parse();
    run_program(options)
}
