use super::meta::META_SCHEMA_ID;
use super::selectors::Selectors;
use crate::schemas::loader::Loader;
use crate::schemas::manager::ManagerWeak;
use std::collections::HashMap;
use url::Url;

#[derive(Default)]
pub struct LoaderImpl<'a> {
    root_node_map: HashMap<Url, serde_json::Value>,
    manager: ManagerWeak<'a>,
}

impl<'a> LoaderImpl<'a> {
    pub fn new(manager: ManagerWeak<'a>) -> Self {
        Self {
            manager,
            ..Default::default()
        }
    }

    fn load_from_node(
        &mut self,
        node: &serde_json::Value,
        node_url: &Url,
        retrieval_url: &Url,
    ) -> Result<Url, &'static str> {
        let manager = self.manager.upgrade().unwrap();

        if let Some(node_ref) = node.select_ref() {
            let node_ref_url = node_url
                .join(node_ref)
                .map_err(|_error_| "could not build node_ref_url")?;
            let mut retrieval_ref_url = retrieval_url
                .join(node_ref)
                .map_err(|_error_| "could not build retrieval_ref_url")?;
            retrieval_ref_url.set_fragment(None);

            return manager.borrow_mut().load_from_url(
                &node_ref_url,
                &retrieval_ref_url,
                META_SCHEMA_ID.into(),
            );
        }

        Ok(node_url.clone())
    }

    fn load_from_root_node(
        &mut self,
        node: serde_json::Value,
        node_url: &Url,
        retrieval_url: &Url,
    ) -> Result<Url, &'static str> {
        let node_url = Self::get_root_node_url(&node, node_url)?;

        self.load_from_sub_nodes(&node, &node_url, retrieval_url, "")?;

        self.root_node_map.insert(node_url.clone(), node);

        Ok(node_url)
    }

    fn load_from_sub_nodes(
        &mut self,
        node: &serde_json::Value,
        node_url: &Url,
        retrieval_url: &Url,
        pointer: &str,
    ) -> Result<(), &'static str> {
        for (sub_pointer, sub_node) in node.select_sub_nodes(pointer) {
            self.load_from_node(node, node_url, retrieval_url)?;

            self.load_from_sub_nodes(sub_node, node_url, retrieval_url, sub_pointer.as_str())?;
        }

        Ok(())
    }

    fn get_root_node_url(
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

    fn load_from_root_node(
        &mut self,
        node: serde_json::Value,
        node_url: &Url,
        retrieval_url: &Url,
    ) -> Result<Url, &'static str> {
        self.load_from_root_node(node, node_url, retrieval_url)
    }
}
