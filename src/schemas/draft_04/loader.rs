use super::meta::META_SCHEMA_ID;
use super::selectors::Selectors;
use crate::schemas::loader::Loader;
use crate::schemas::manager::ManagerWeak;
use crate::schemas::meta::MetaSchemaId;
use std::collections::HashMap;
use url::Url;

#[derive(Default)]
pub struct LoaderImpl<'a> {
    _root_node_map: HashMap<Url, serde_json::Value>,
    _manager: ManagerWeak<'a>,
}

impl<'a> LoaderImpl<'a> {
    pub fn new(manager: ManagerWeak<'a>) -> Self {
        Self {
            _manager: manager,
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
        _node: &serde_json::Value,
        _node_url: &'a Url,
        _retrieval_url: &'a Url,
        _referencing_url: Option<&'a Url>,
        _default_meta_schema_id: MetaSchemaId,
    ) -> Result<Url, &'static str> {
        todo!()
    }
}
