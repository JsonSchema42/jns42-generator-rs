use super::meta::META_SCHEMA_ID;
use super::selectors::Selectors;
use crate::schemas::loader::Loader;

pub struct LoaderImpl {}

impl LoaderImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Loader for LoaderImpl {
    fn is_schema_root_node(&self, node: &serde_json::Value) -> bool {
        if let Some(schema) = node.schema() {
            return schema == META_SCHEMA_ID;
        }
        false
    }
}
