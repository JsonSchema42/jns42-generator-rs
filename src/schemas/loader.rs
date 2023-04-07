use url::Url;

pub trait Loader<'a> {
    fn is_schema_root_node(&self, node: &serde_json::Value) -> bool;

    fn load_from_root_node(
        &mut self,
        node: serde_json::Value,
        node_url: &Url,
        retrieval_url: &Url,
    ) -> Result<Url, &'static str>;
}
