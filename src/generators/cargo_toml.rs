pub struct CargoTomlGenerator;

impl CargoTomlGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_file_content(
        &self,
        package_name: &str,
        package_version: &str,
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
