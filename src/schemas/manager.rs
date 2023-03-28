use std::fs::File;
use url::Url;

use super::meta::MetaSchemaId;

#[derive(Default)]
pub struct Manager {}

impl Manager {
    pub fn new() -> Self {
        Manager {}
    }

    pub fn load_from_url(
        &self,
        node_url: &Url,
        retrieval_url: &Url,
        referencing_url: Option<&Url>,
        default_meta_schema_id: MetaSchemaId,
    ) -> Result<(), &'static str> {
        let value = self.fetch_root_node_from_url(retrieval_url)?;

        println!("{:?}", value);

        Ok(())
    }

    pub fn load_from_node(&self, url: &Url) -> Result<(), &'static str> {
        Ok(())
    }

    pub fn fetch_root_node_from_url(&self, url: &Url) -> Result<serde_json::Value, &'static str> {
        match url.scheme() {
            "file" => {
                let path = url.path();
                let reader = File::open(path).or(Err("error reading file"))?;

                let value: serde_json::Value =
                    serde_json::from_reader(reader).or(Err("error deserializing file content"))?;

                Ok(value)
            }
            _ => Err("not supported"),
        }
    }
}
