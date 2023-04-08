use crate::schemas::LoaderContext;

pub struct CargoTomlGenerator<'a> {
    schema_loader: &'a LoaderContext<'a>,
}

impl<'a> CargoTomlGenerator<'a> {
    pub fn new(schema_loader: &'a LoaderContext<'a>) -> Self {
        Self { schema_loader }
    }

    pub fn generate_file_content(
        &self,
        package_name: &'a str,
        package_version: &'a str,
    ) -> Result<String, &'static str> {
        let mut package_map = toml::map::Map::new();

        package_map.insert(
            "name".to_owned(),
            toml::Value::String(package_name.to_owned()),
        );
        package_map.insert(
            "version".to_owned(),
            toml::Value::String(package_version.to_owned()),
        );

        let package = toml::Value::Table(package_map);

        let mut manifest_map = toml::map::Map::new();

        manifest_map.insert("package".to_owned(), package);

        let manifest = toml::Value::Table(manifest_map);

        toml::ser::to_string_pretty(&manifest).or(Err("serializtion failed"))
    }
}
