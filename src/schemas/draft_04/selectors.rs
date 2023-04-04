pub trait Selectors {
    fn select_schema(&self) -> Option<&str>;
    fn select_id(&self) -> Option<&str>;
    fn select_sub_node_entries(&self, node_pointer: &str) -> Vec<(&str, &serde_json::Value)>;
}

impl Selectors for serde_json::Value {
    fn select_schema(&self) -> Option<&str> {
        self.as_object()?.get("$schema")?.as_str()
    }

    fn select_id(&self) -> Option<&str> {
        self.as_object()?.get("$id")?.as_str()
    }

    fn select_sub_node_entries(&self, node_pointer: &str) -> Vec<(&str, &serde_json::Value)> {
        todo!()
    }
}
