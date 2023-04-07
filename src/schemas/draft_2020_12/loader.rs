use super::meta::META_SCHEMA_ID;
use super::selectors::Selectors;
use crate::schemas::loader::Loader;
use crate::schemas::manager::ManagerWeak;
use std::collections::HashMap;
use url::Url;

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
        node: serde_json::Value,
        node_url: &Url,
    ) -> Result<(), &'static str> {
        self.root_node_map.insert(node_url.clone(), node);

        Ok(())
    }

    pub fn get_sub_urls(
        &mut self,
        node: &serde_json::Value,
        node_url: &Url,
        retrieval_url: &Url,
    ) -> Result<Vec<(Url, Url)>, &'static str> {
        let node_url = self.get_root_node_url(node, node_url)?;
        let mut result = Vec::new();

        for node_ref in node
            .select_all_sub_nodes("")
            .iter()
            .filter_map(|(_sub_pointer, sub_node)| sub_node.select_ref())
        {
            let node_ref_url = node_url
                .join(node_ref)
                .map_err(|_error_| "could not build node_ref_url")?;
            let mut retrieval_ref_url = retrieval_url
                .join(node_ref)
                .map_err(|_error_| "could not build retrieval_ref_url")?;
            retrieval_ref_url.set_fragment(None);

            result.push((node_ref_url, retrieval_ref_url));
        }

        Ok(result)
    }

    fn get_root_node_url(
        &self,
        node: &serde_json::Value,
        default_node_url: &Url,
    ) -> Result<Url, &'static str> {
        let node_url;

        let id = node.select_id();
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
        if let Some(schema) = node.select_schema() {
            return schema == META_SCHEMA_ID;
        }

        false
    }

    fn load_root_node(
        &mut self,
        node: serde_json::Value,
        node_url: &Url,
    ) -> Result<(), &'static str> {
        self.load_from_root_node(node, node_url)
    }

    fn get_sub_urls(
        &mut self,
        node: &serde_json::Value,
        node_url: &Url,
        retrieval_url: &Url,
    ) -> Result<Vec<(Url, Url)>, &'static str> {
        self.get_sub_urls(node, node_url, retrieval_url)
    }

    fn get_root_node_url(
        &self,
        node: &serde_json::Value,
        default_node_url: &Url,
    ) -> Result<Url, &'static str> {
        self.get_root_node_url(node, default_node_url)
    }
}
