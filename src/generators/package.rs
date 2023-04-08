use crate::schemas::LoaderContext;
use std::{fs, path::PathBuf};

use super::{cargo_toml::CargoTomlGenerator, models_rs::ModelsRsGenerator};

pub struct PackageGenerator<'a> {
    schema_loader: &'a LoaderContext<'a>,
    cargo_toml_generator: CargoTomlGenerator<'a>,
    models_rs_generator: ModelsRsGenerator<'a>,
}

impl<'a> PackageGenerator<'a> {
    pub fn new(schema_loader: &'a LoaderContext<'a>) -> Self {
        Self {
            schema_loader,
            cargo_toml_generator: CargoTomlGenerator::new(schema_loader),
            models_rs_generator: ModelsRsGenerator::new(schema_loader),
        }
    }

    pub fn generate_package(
        &self,
        package_name: &str,
        package_version: &str,
        package_directory: &PathBuf,
    ) -> Result<(), &'static str> {
        fs::create_dir_all(package_directory).or(Err("create directory failed"))?;

        let content = self
            .cargo_toml_generator
            .generate_file_content(package_name, package_version)?;

        fs::write(package_directory.join("Cargo.toml"), content)
            .or(Err("write Cargo.toml fails"))?;

        Ok(())
    }
}
