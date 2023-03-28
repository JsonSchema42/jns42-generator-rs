use crate::meta_schema_id::*;
use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
pub struct CommandOptions {
    pub schema_url: Url,

    #[arg(long, default_value_t = MetaSchemaId::Draft202012)]
    pub default_meta_schema_url: MetaSchemaId,

    #[arg(long)]
    pub package_directory: String,

    #[arg(long)]
    pub package_name: String,

    #[arg(long)]
    pub package_version: String,

    #[arg(long)]
    pub generate_test: bool,

    #[arg(long, default_value_t = 0)]
    pub unique_name_seed: usize,
}

pub fn run_command(options: CommandOptions) {
    let CommandOptions {
        schema_url,
        default_meta_schema_url,
        package_directory,
        package_name,
        package_version,
        generate_test,
        unique_name_seed,
    } = options;

    println!("{:?}", schema_url);
    println!("{:?}", default_meta_schema_url);
    println!("{:?}", package_directory);
    println!("{:?}", package_name);
    println!("{:?}", package_version);
    println!("{:?}", generate_test);
    println!("{:?}", unique_name_seed);
}
