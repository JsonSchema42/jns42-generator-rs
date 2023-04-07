use super::loader::SchemaNode;

pub trait JsonValueSelectors {
    fn schema(&self) -> Option<&str>;
}

impl JsonValueSelectors for serde_json::Value {
    fn schema(&self) -> Option<&str> {
        self.as_object()?.get("$schema")?.as_str()
    }
}

pub trait SchemaNodeSelectors {
    fn id(&self) -> Option<&str>;
}

impl SchemaNodeSelectors for SchemaNode {
    fn id(&self) -> Option<&str> {
        self.as_object()?.get("$id")?.as_str()
    }
}
