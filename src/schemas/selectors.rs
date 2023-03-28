pub trait Selectors {
    fn schema(&self) -> Option<&str>;
}

impl Selectors for serde_json::Value {
    fn schema(&self) -> Option<&str> {
        self.as_object()?.get("$schema")?.as_str()
    }
}
