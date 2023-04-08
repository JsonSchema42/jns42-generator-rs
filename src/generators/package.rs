use url::Url;

use crate::{schemas::InterpreterContext, utils::Namer};
use std::{fs, path::PathBuf};

use super::{
    cargo_toml::CargoTomlGenerator, lib_rs::LibRsGenerator, models_rs::ModelsRsGenerator,
    validators_rs::ValidatorsRsGenerator,
};

pub struct PackageGenerator<'a> {
    cargo_toml_generator: CargoTomlGenerator,
    lib_rs_generator: LibRsGenerator,
    models_rs_generator: ModelsRsGenerator<'a>,
    validators_rs_generator: ValidatorsRsGenerator<'a>,
}

impl<'a> PackageGenerator<'a> {
    pub fn new(schema_loader: &'a InterpreterContext<'a>, namer: &'a Namer<Url>) -> Self {
        Self {
            cargo_toml_generator: CargoTomlGenerator::new(),
            lib_rs_generator: LibRsGenerator::new(),
            models_rs_generator: ModelsRsGenerator::new(schema_loader, namer),
            validators_rs_generator: ValidatorsRsGenerator::new(schema_loader, namer),
        }
    }

    pub fn generate_package(
        &self,
        package_name: &str,
        package_version: &str,
        package_directory: &PathBuf,
    ) -> Result<(), &'static str> {
        fs::create_dir_all(package_directory).or(Err("create directory failed"))?;

        {
            let content = self
                .cargo_toml_generator
                .generate_file_content(package_name, package_version)?;

            fs::write(package_directory.join("Cargo.toml"), content)
                .or(Err("write Cargo.toml fails"))?;
        }

        {
            let content = self.lib_rs_generator.generate_file_content()?;

            fs::write(package_directory.join("lib.rs"), content).or(Err("write lib.rs fails"))?;
        }

        {
            let content = self.models_rs_generator.generate_file_content()?;

            fs::write(package_directory.join("models.rs"), content)
                .or(Err("write models.rs fails"))?;
        }

        {
            let content = self.validators_rs_generator.generate_file_content()?;

            fs::write(package_directory.join("validators.rs"), content)
                .or(Err("write validators.rs fails"))?;
        }

        Ok(())
    }
}
