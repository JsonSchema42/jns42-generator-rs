mod meta_schema_id;
mod program;
mod schema;

use clap::Parser;
use program::ProgramOptions;

fn main() {
    let options = ProgramOptions::parse();

    match options.command {
        program::ProgramCommands::Package {
            schema_url,
            default_meta_schema_url,
            package_directory,
            package_name,
            package_version,
            generate_test,
            unique_name_seed,
        } => {
            println!("{:?}", schema_url);
            println!("{:?}", default_meta_schema_url);
            println!("{:?}", package_directory);
            println!("{:?}", package_name);
            println!("{:?}", package_version);
            println!("{:?}", generate_test);
            println!("{:?}", unique_name_seed);
        }
    }
}
