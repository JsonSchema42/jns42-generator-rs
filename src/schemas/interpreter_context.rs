use super::interpreter_strategy::InterpreterStrategyBox;
use super::meta_schema_id::MetaSchemaId;
use crate::schemas::{draft_04, draft_06, draft_07, draft_2019_09, draft_2020_12};
use crate::utils::ValueRc;
use std::rc::Rc;
use std::{collections::HashMap, fs::File};
use url::Url;

pub struct InterpreterContext<'a> {
    strategies: HashMap<MetaSchemaId, InterpreterStrategyBox<'a>>,
    retrieval_root_node_map: HashMap<Url, Url>,
    root_node_retrieval_map: HashMap<Url, Url>,
    root_node_meta_schema_id_map: HashMap<Url, MetaSchemaId>,
    node_meta_schema_id_map: HashMap<Url, MetaSchemaId>,
}

impl<'a> InterpreterContext<'a> {
    pub fn new() -> Self {
        Self {
            strategies: vec![
                (
                    MetaSchemaId::Draft202012,
                    Box::new(draft_2020_12::Interpreter::new()) as InterpreterStrategyBox,
                ),
                (
                    MetaSchemaId::Draft201909,
                    Box::new(draft_2019_09::Interpreter::new()),
                ),
                (
                    MetaSchemaId::Draft07,
                    Box::new(draft_07::Interpreter::new()),
                ),
                (
                    MetaSchemaId::Draft06,
                    Box::new(draft_06::Interpreter::new()),
                ),
                (
                    MetaSchemaId::Draft04,
                    Box::new(draft_04::Interpreter::new()),
                ),
            ]
            .into_iter()
            .collect(),

            retrieval_root_node_map: Default::default(),
            root_node_retrieval_map: Default::default(),
            root_node_meta_schema_id_map: Default::default(),
            node_meta_schema_id_map: Default::default(),
        }
    }

    pub fn load_root_node(
        &mut self,
        node: Rc<ValueRc>,
        node_url: &Url,
        default_meta_schema_id: MetaSchemaId,
    ) -> Result<(), &'static str> {
        let meta_schema_id = self.discover_meta_schema_id(node.clone(), default_meta_schema_id);

        let strategy = self.strategies.get_mut(&meta_schema_id).unwrap();

        let root_node_url = strategy.get_root_node_url(node.clone(), node_url)?;

        strategy.load_root_node(node, &root_node_url)?;
        for node_url in strategy.index_root_node(&root_node_url)? {
            self.node_meta_schema_id_map
                .insert(node_url, meta_schema_id);
        }

        Ok(())
    }

    pub fn load_from_url(
        &mut self,
        node_url: &Url,
        retrieval_url: &Url,
        default_meta_schema_id: MetaSchemaId,
    ) -> Result<(), &'static str> {
        if self.retrieval_root_node_map.contains_key(retrieval_url) {
            return Ok(());
        }

        let root_node = Self::fetch_json_from_url(retrieval_url)?;

        let meta_schema_id =
            self.discover_meta_schema_id(root_node.clone(), default_meta_schema_id);

        let strategy = self.strategies.get(&meta_schema_id).unwrap();

        let node_url = strategy.get_root_node_url(root_node.clone(), node_url)?;

        self.retrieval_root_node_map
            .insert(retrieval_url.clone(), node_url.clone());
        self.root_node_retrieval_map
            .insert(node_url.clone(), retrieval_url.clone());
        self.root_node_meta_schema_id_map
            .insert(node_url.clone(), meta_schema_id);

        for (sub_node_url, sub_retrieval_url) in
            strategy.get_sub_node_urls(root_node.clone(), &node_url, retrieval_url)?
        {
            self.load_from_url(&sub_node_url, &sub_retrieval_url, meta_schema_id)?;
        }

        self.load_root_node(root_node, &node_url, default_meta_schema_id)?;

        Ok(())
    }

    pub fn get_all_node_urls(&self) -> Vec<Url> {
        self.node_meta_schema_id_map.keys().cloned().collect()
    }

    pub fn get_node_model_name(&self, node_url: &Url) -> String {
        let segments = node_url.path_segments();

        if let Some(segments) = segments {
            if let Some(segment) = segments.last() {
                return segment.to_owned();
            }
        }

        "Model".to_owned()
    }

    fn discover_meta_schema_id(
        &self,
        node: Rc<ValueRc>,
        default_meta_schema_id: MetaSchemaId,
    ) -> MetaSchemaId {
        for (schema_id, strategy) in self.strategies.iter() {
            if strategy.is_schema_root_node(node.clone()) {
                return *schema_id;
            }
        }

        default_meta_schema_id
    }

    fn fetch_json_from_url(url: &Url) -> Result<Rc<ValueRc>, &'static str> {
        match url.scheme() {
            "file" => {
                let path = url.path();
                let reader = File::open(path).or(Err("error reading file"))?;

                let value: ValueRc =
                    serde_json::from_reader(reader).or(Err("error deserializing file content"))?;
                let value = Rc::new(value);

                Ok(value)
            }
            _ => Err("not supported"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_manager() {
        let _manager = InterpreterContext::new();
    }
}
