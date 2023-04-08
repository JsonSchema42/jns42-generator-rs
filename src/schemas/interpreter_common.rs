use url::Url;

pub enum InterpreterModelInfo {
    Null,
    Boolean,
    Integer,
    Number,
    String,
    Array,
    Object,
}

pub trait InterpreterCommon {
    fn get_node_model_info(&self, node_url: &Url) -> Option<InterpreterModelInfo>;
}
