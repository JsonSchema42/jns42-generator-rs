use super::meta::META_SCHEMA_ID;
use super::selectors::Selectors;
use crate::schemas::{InterpreterCommon, InterpreterModelInfo, InterpreterStrategy};
use crate::utils::ValueRc;
use std::collections::BTreeMap;
use std::rc::Rc;
use url::Url;

pub struct Interpreter {
    _root_node_map: BTreeMap<Url, serde_json::Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            _root_node_map: Default::default(),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl InterpreterStrategy for Interpreter {
    fn is_schema_root_node(&self, node: Rc<ValueRc>) -> bool {
        if let Some(schema) = node.select_schema() {
            return schema == META_SCHEMA_ID;
        }

        false
    }

    fn load_root_node(&mut self, _node: Rc<ValueRc>, _node_url: &Url) -> Result<(), &'static str> {
        todo!()
    }

    fn index_root_node(&mut self, _node_url: &Url) -> Result<Vec<Url>, &'static str> {
        todo!()
    }

    fn get_sub_node_urls(
        &self,
        _node: Rc<ValueRc>,
        _node_url: &Url,
        _retrieval_url: &Url,
    ) -> Result<Vec<(Url, Url)>, &'static str> {
        todo!()
    }

    fn get_root_node_url(
        &self,
        _node: Rc<ValueRc>,
        _default_node_url: &Url,
    ) -> Result<Url, &'static str> {
        todo!()
    }
}

impl InterpreterCommon for Interpreter {
    fn get_node_model_info(&self, _node_url: &Url) -> Option<InterpreterModelInfo> {
        todo!()
    }
}
