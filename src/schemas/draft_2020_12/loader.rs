use super::meta::META_SCHEMA_ID;
use super::selectors::{JsonValueSelectors, SchemaNodeSelectors};
use crate::schemas::loader::Loader;
use crate::schemas::manager::ManagerWeak;
use std::collections::HashMap;
use url::Url;

pub type SchemaNode = serde_json::Value;

#[derive(Default)]
pub struct LoaderImpl<'a> {
    root_node_map: HashMap<Url, serde_json::Value>,
    _manager: ManagerWeak<'a>,
}

impl<'a> LoaderImpl<'a> {
    pub fn new(manager: ManagerWeak<'a>) -> Self {
        Self {
            _manager: manager,
            ..Default::default()
        }
    }

    fn load_from_root_node(
        &mut self,
        node: SchemaNode,
        node_url: &'a Url,
    ) -> Result<Url, &'static str> {
        let node_url = Self::get_root_node_url(&node, node_url)?;

        self.root_node_map.insert(node_url.clone(), node);

        Ok(node_url)
    }

    fn get_root_node_url(node: &SchemaNode, default_node_url: &Url) -> Result<Url, &'static str> {
        let node_url;

        let id = node.id();
        if let Some(id) = id {
            node_url = id.parse().map_err(|_error| "could not parse id")?;
        } else {
            node_url = default_node_url.clone();
        }

        Ok(node_url)
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
        &mut self,
        node: serde_json::Value,
        node_url: &'a Url,
    ) -> Result<Url, &'static str> {
        self.load_from_root_node(node, node_url)
    }
}
