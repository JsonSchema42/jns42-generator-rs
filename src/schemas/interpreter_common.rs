use url::Url;

pub enum InterpreterModelInfo {
    Null,
    Boolean,
    Integer,
    Number,
    String,
    Array,
    Object(Vec<InterpreterModelPropertyInfo>),
}

pub struct InterpreterModelPropertyInfo {
    pub node_url: Url,
    pub required: bool,
    pub name: String,
}

pub trait InterpreterCommon {
    fn get_node_model_info(&self, node_url: &Url) -> Option<InterpreterModelInfo>;
}
