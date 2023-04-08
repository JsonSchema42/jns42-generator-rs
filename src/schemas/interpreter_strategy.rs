use super::InterpreterCommon;
use crate::utils::ValueRc;
use std::rc::Rc;
use url::Url;

pub type InterpreterStrategyBox<'a> = Box<dyn InterpreterStrategy + 'a>;

pub trait InterpreterStrategy: InterpreterCommon {
    fn is_schema_root_node(&self, node: Rc<ValueRc>) -> bool;

    fn load_root_node(&mut self, node: Rc<ValueRc>, node_url: &Url) -> Result<(), &'static str>;

    fn index_root_node(&mut self, node_url: &Url) -> Result<Vec<Url>, &'static str>;

    fn get_sub_node_urls(
        &self,
        node: Rc<ValueRc>,
        node_url: &Url,
        retrieval_url: &Url,
    ) -> Result<Vec<(Url, Url)>, &'static str>;

    fn get_root_node_url(
        &self,
        node: Rc<ValueRc>,
        default_node_url: &Url,
    ) -> Result<Url, &'static str>;
}
