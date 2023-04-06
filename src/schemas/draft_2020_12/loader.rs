use super::meta::META_SCHEMA_ID;
use super::selectors::Selectors;
use crate::schemas::loader::Loader;
use crate::schemas::manager::Manager;
use crate::schemas::meta::MetaSchemaId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;
use url::Url;

#[derive(Default)]
pub struct LoaderImpl<'a> {
    root_node_map: HashMap<Url, serde_json::Value>,
    manager: Weak<RefCell<Manager<'a>>>,
}

impl<'a> LoaderImpl<'a> {
    pub fn new(manager: Weak<RefCell<Manager<'a>>>) -> Self {
        Self {
            manager,
            ..Default::default()
        }
    }
}

impl<'a> Loader<'a> for LoaderImpl<'a> {
    fn is_schema_root_node(&self, node: &serde_json::Value) -> bool {
        if let Some(schema) = node.schema() {
            return schema == META_SCHEMA_ID;
        }
        false
    }

    fn load_from_root_node(
        &self,
        node: &serde_json::Value,
        node_url: &'a Url,
        retrieval_url: &'a Url,
        referencing_url: Option<&'a Url>,
        default_meta_schema_id: MetaSchemaId,
    ) -> Result<Url, &'static str> {
        todo!()
    }
}
