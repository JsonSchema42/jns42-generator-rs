pub fn generate_file_content(
    package_name: &str,
    package_version: &str,
) -> Result<String, &'static str> {
    let package_table = toml::Value::Table({
        let mut map = toml::map::Map::new();
        map.insert(
            "name".to_owned(),
            toml::Value::String(package_name.to_owned()),
        );
        map.insert(
            "version".to_owned(),
            toml::Value::String(package_version.to_owned()),
        );
        map.insert("edition".to_owned(), toml::Value::String("2021".to_owned()));
        map
    });

    let dependencies_table = toml::Value::Table({
        let mut map = toml::map::Map::new();
        map.insert(
            "serde".to_owned(),
            toml::Value::Table({
                let mut map = toml::map::Map::new();
                map.insert("version".to_owned(), toml::Value::String("1.0".to_owned()));
                map.insert(
                    "features".to_owned(),
                    toml::Value::Array(vec![toml::Value::String("derive".to_owned())]),
                );
                map
            }),
        );
        map
    });

    let lib_table = toml::Value::Table({
        let mut map = toml::map::Map::new();
        map.insert("path".to_owned(), toml::Value::String("lib.rs".to_owned()));
        map
    });

    let manifest_table = toml::Value::Table({
        let mut map = toml::map::Map::new();
        map.insert("package".to_owned(), package_table);
        map.insert("dependencies".to_owned(), dependencies_table);
        map.insert("lib".to_owned(), lib_table);
        map
    });

    toml::ser::to_string_pretty(&manifest_table).or(Err("serialization failed"))
}
