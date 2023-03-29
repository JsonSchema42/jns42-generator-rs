use super::meta::META_SCHEMA_ID;
use super::selectors::Selectors;
use crate::schemas::loader::Loader;
use crate::schemas::meta::MetaSchemaId;
use url::Url;

pub struct LoaderImpl {}

impl LoaderImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> Loader<'a> for LoaderImpl {
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
    ) -> Result<&'a Url, &'static str> {
        todo!()
    }
}
