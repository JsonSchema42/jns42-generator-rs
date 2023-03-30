use super::{manager::Manager, meta::MetaSchemaId};
use std::{cell::RefCell, rc::Rc};
use url::Url;

pub trait Loader<'a> {
    fn is_schema_root_node(&self, node: &serde_json::Value) -> bool;

    fn load_from_root_node(
        &self,
        node: &serde_json::Value,
        node_url: &'a Url,
        retrieval_url: &'a Url,
        referencing_url: Option<&'a Url>,
        default_meta_schema_id: MetaSchemaId,
    ) -> Result<&'a Url, &'static str>;
}