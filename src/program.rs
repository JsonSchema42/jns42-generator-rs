use crate::meta_schema_id::*;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ProgramOptions {
    #[command(subcommand)]
    pub command: ProgramCommands,
}

#[derive(Subcommand, Debug)]
pub enum ProgramCommands {
    Package {
        schema_url: String,

        #[arg(long, default_value_t = MetaSchemaId::Draft202012)]
        default_meta_schema_url: MetaSchemaId,

        #[arg(long)]
        package_directory: String,

        #[arg(long)]
        package_name: String,

        #[arg(long)]
        package_version: String,

        #[arg(long)]
        generate_test: bool,

        #[arg(long, default_value_t = 0)]
        unique_name_seed: usize,
    },
}
