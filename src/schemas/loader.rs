pub trait Loader {
    fn is_schema_root_node(&self, node: &serde_json::Value) -> bool;
}
