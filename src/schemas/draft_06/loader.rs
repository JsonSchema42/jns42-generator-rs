use super::meta::META_SCHEMA_ID;
use super::selectors::Selectors;
use crate::schemas::loader::Loader;
use crate::schemas::manager::ManagerWeak;
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
        if let Some(schema) = node.select_schema() {
            return schema == META_SCHEMA_ID;
        }

        false
    }

    fn load_root_node(
        &mut self,
        _node: serde_json::Value,
        _node_url: &Url,
    ) -> Result<(), &'static str> {
        todo!()
    }

    fn get_sub_urls(
        &mut self,
        _node: &serde_json::Value,
        _node_url: &Url,
        _retrieval_url: &Url,
    ) -> Result<Vec<(Url, Url)>, &'static str> {
        todo!()
    }

    fn get_root_node_url(
        &self,
        _node: &serde_json::Value,
        _default_node_url: &Url,
    ) -> Result<Url, &'static str> {
        todo!()
    }
}
