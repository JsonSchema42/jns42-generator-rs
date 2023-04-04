use super::meta::META_SCHEMA_ID;
use super::selectors::Selectors;
use crate::schemas::loader::Loader;
use crate::schemas::manager::Manager;
use crate::schemas::meta::MetaSchemaId;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use url::Url;

struct SchemaLoaderRootNodeItem<'a> {
    node: &'a serde_json::Value,
    node_url: &'a Url,
    referencing_node_url: Option<&'a Url>,
}

#[derive(Default)]
pub struct LoaderImpl<'a> {
    manager: Weak<RefCell<Option<Manager<'a>>>>,
    root_node_map: HashMap<&'a Url, SchemaLoaderRootNodeItem<'a>>,
}

impl<'a> LoaderImpl<'a> {
    pub fn new(manager: Weak<RefCell<Option<Manager<'a>>>>) -> Self {
        Self {
            manager,
            ..Default::default()
        }
    }
}

impl<'a> LoaderImpl<'a> {
    fn load_from_url(
        &mut self,
        node: &'a serde_json::Value,
        node_url: &'a Url,
        retrieval_url: &'a Url,
    ) {
        todo!()
    }

    fn load_from_sub_nodes(
        &mut self,
        node: &'a serde_json::Value,
        node_url: &'a Url,
        retrieval_url: &'a Url,
        node_pointer: &'a str,
    ) {
        self.load_from_url(node, node_url, retrieval_url);

        for (sub_node_pointer, sub_node) in node.select_sub_node_entries(node_pointer).into_iter() {
            let sub_node_rc = Rc::new(sub_node);
            self.load_from_sub_nodes(sub_node, node_url, retrieval_url, &sub_node_pointer);
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

    fn load_from_root_node(
        &mut self,
        node: &'a serde_json::Value,
        node_url: &'a Url,
        retrieval_url: &'a Url,
        referencing_node_url: Option<&'a Url>,
        default_meta_schema_id: MetaSchemaId,
    ) -> Result<Cow<'a, Url>, &'static str> {
        let mut node_url_cow = Cow::Borrowed(node_url);
        let maybe_node_id = node.select_id();
        if let Some(node_id) = maybe_node_id {
            node_url_cow = Cow::Owned(Url::parse(node_id).unwrap());
        }

        if self.root_node_map.contains_key(node_url_cow.as_ref()) {
            return Ok(node_url_cow);
        }

        let item = SchemaLoaderRootNodeItem {
            node,
            node_url,
            referencing_node_url,
        };

        self.root_node_map.insert(node_url, item);

        // TODO register with manager

        // TODO load from subnodes

        Ok(node_url_cow)
    }
}
