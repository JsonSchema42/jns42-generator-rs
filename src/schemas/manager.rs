use super::{loader::Loader, meta::MetaSchemaId};
use crate::schemas;
use std::{collections::HashMap, fs::File};
use url::Url;

#[derive(Default)]
pub struct Manager {
    loaders: HashMap<MetaSchemaId, Box<dyn Loader>>,
}

impl Manager {
    pub fn new() -> Self {
        let mut loaders: HashMap<MetaSchemaId, Box<dyn Loader>> = HashMap::new();

        loaders.insert(
            MetaSchemaId::Draft202012,
            Box::new(schemas::draft_2020_12::loader::LoaderImpl::new()),
        );
        loaders.insert(
            MetaSchemaId::Draft201909,
            Box::new(schemas::draft_2019_09::loader::LoaderImpl::new()),
        );
        loaders.insert(
            MetaSchemaId::Draft07,
            Box::new(schemas::draft_07::loader::LoaderImpl::new()),
        );
        loaders.insert(
            MetaSchemaId::Draft06,
            Box::new(schemas::draft_06::loader::LoaderImpl::new()),
        );
        loaders.insert(
            MetaSchemaId::Draft04,
            Box::new(schemas::draft_04::loader::LoaderImpl::new()),
        );

        Manager { loaders }
    }

    pub fn load_from_node(
        &self,
        node: &serde_json::Value,
        nodeUrl: &Url,
        retrievelUrl: &Url,
        referencing_url: Option<&Url>,
        default_meta_schema_id: &MetaSchemaId,
    ) -> Result<(), &'static str> {
        let mut schema_id = self.discover_schema_id(node);
        if schema_id == &MetaSchemaId::Unknown {
            schema_id = default_meta_schema_id;
        }

        Ok(())
    }

    pub fn load_from_url(
        &self,
        node_url: &Url,
        retrieval_url: &Url,
        referencing_url: Option<&Url>,
        default_meta_schema_id: &MetaSchemaId,
    ) -> Result<(), &'static str> {
        let value = self.fetch_root_node_from_url(retrieval_url)?;

        println!("{:?}", value);

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

    pub fn discover_schema_id(&self, node: &serde_json::Value) -> &MetaSchemaId {
        for (schema_id, loader) in self.loaders.iter() {
            if loader.is_schema_root_node(node) {
                return schema_id;
            }
        }

        &MetaSchemaId::Unknown
    }
}
