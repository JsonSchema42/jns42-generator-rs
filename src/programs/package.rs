use crate::utils::Namer;
use crate::{
    generators::PackageGenerator,
    schemas::{LoaderContext, MetaSchemaId},
};
use clap::Parser;
use std::path::PathBuf;
use url::Url;

#[derive(Parser, Debug)]
pub struct CommandOptions {
    pub schema_url: Url,

    #[arg(long, default_value_t = MetaSchemaId::Draft202012)]
    pub default_meta_schema_url: MetaSchemaId,

    #[arg(long)]
    pub package_directory: PathBuf,

    #[arg(long)]
    pub package_name: String,

    #[arg(long)]
    pub package_version: String,

    #[arg(long)]
    pub generate_test: bool,

    #[arg(long, default_value_t = 0)]
    pub unique_name_seed: usize,
}

pub fn run_command(options: CommandOptions) -> Result<(), &'static str> {
    let CommandOptions {
        schema_url,
        default_meta_schema_url,
        package_name,
        package_version,
        package_directory,
        ..
    } = options;

    let mut _namer = Namer::<Url>::new(0);
    let mut loader_context = LoaderContext::new();

    loader_context.load_from_url(&schema_url, &schema_url, default_meta_schema_url)?;

    let package_generator = PackageGenerator::new(&loader_context);

    package_generator.generate_package(&package_name, &package_version, &package_directory)?;

    Ok(())
}
