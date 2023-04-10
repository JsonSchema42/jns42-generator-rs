use super::meta::META_SCHEMA_ID;
use super::selectors::Selectors;
use crate::schemas::interpreter_common::InterpreterModelPropertyInfo;
use crate::schemas::interpreter_strategy::InterpreterStrategy;
use crate::schemas::{InterpreterCommon, InterpreterModelInfo};
use crate::utils::ValueRc;
use std::collections::{BTreeMap, HashSet};
use std::rc::Rc;
use url::Url;

pub struct Interpreter {
    root_node_map: BTreeMap<Url, Rc<ValueRc>>,
    node_map: BTreeMap<Url, Rc<ValueRc>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            root_node_map: Default::default(),
            node_map: Default::default(),
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

    fn load_root_node(&mut self, node: Rc<ValueRc>, node_url: &Url) -> Result<(), &'static str> {
        if self.root_node_map.insert(node_url.clone(), node).is_some() {
            return Err("root_node already present");
        }

        Ok(())
    }

    fn index_root_node(&mut self, root_node_url: &Url) -> Result<Vec<Url>, &'static str> {
        let mut result = Vec::new();

        let root_node = self
            .root_node_map
            .get(root_node_url)
            .ok_or("root_node not found")?;

        for (sub_pointer, sub_node) in root_node.select_all_sub_nodes_and_self("").into_iter() {
            let sub_node_url = root_node_url
                .join(format!("#{}", sub_pointer).as_str())
                .map_err(|_error| "could not build sub_node_url")?;

            self.node_map.insert(sub_node_url.clone(), sub_node);
            result.push(sub_node_url);
        }

        Ok(result)
    }

    fn get_referenced_node_urls(
        &self,
        node: Rc<ValueRc>,
        node_url: &Url,
        retrieval_url: &Url,
    ) -> Result<Vec<(Url, Url)>, &'static str> {
        let mut result = Vec::new();

        for node_ref in node
            .select_all_sub_nodes_and_self("")
            .into_iter()
            .filter_map(|(_pointer, node)| node.select_ref().map(|value| value.to_owned()))
        {
            let ref_node_url = node_url
                .join(node_ref.as_str())
                .map_err(|_error| "could not build node_ref_url")?;
            let mut ref_retrieval_url = retrieval_url
                .join(node_ref.as_str())
                .map_err(|_error| "could not build retrieval_ref_url")?;
            ref_retrieval_url.set_fragment(None);

            result.push((ref_node_url, ref_retrieval_url));
        }

        Ok(result)
    }

    fn get_root_node_url(
        &self,
        node: Rc<ValueRc>,
        default_node_url: &Url,
    ) -> Result<Url, &'static str> {
        let node_url;

        let node_id = node.select_id();
        if let Some(node_id) = node_id {
            node_url = node_id.parse().map_err(|_error| "could not parse id")?;
        } else {
            node_url = default_node_url.clone();
        }

        Ok(node_url)
    }
}

impl InterpreterCommon for Interpreter {
    fn get_node_model_info(&self, node_url: &Url) -> Option<InterpreterModelInfo> {
        let node = self.node_map.get(node_url)?;

        let types = node.select_types()?;

        if types.len() == 1 {
            let first_type = *types.first()?;

            return match first_type {
                "null" => Some(InterpreterModelInfo::Null),
                "boolean" => Some(InterpreterModelInfo::Boolean),
                "integer" => Some(InterpreterModelInfo::Integer),
                "number" => Some(InterpreterModelInfo::Number),
                "string" => Some(InterpreterModelInfo::String),
                "array" => Some(InterpreterModelInfo::Array),
                "object" => {
                    let mut property_infos = Vec::new();

                    let required_property_names: HashSet<_> = node
                        .select_required_property_names()
                        .unwrap_or_default()
                        .into_iter()
                        .collect();
                    for (pointer, name) in node
                        .select_property_names_entries(node_url.fragment().unwrap_or_default())
                        .unwrap_or_default()
                    {
                        let mut node_url = node_url.clone();
                        node_url.set_fragment(if pointer.is_empty() {
                            None
                        } else {
                            Some(pointer.as_str())
                        });
                        let property_info = InterpreterModelPropertyInfo {
                            name: name.to_owned(),
                            required: required_property_names.contains(name),
                            node_url,
                        };

                        property_infos.push(property_info);
                    }

                    Some(InterpreterModelInfo::Object(property_infos))
                }
                _ => Some(InterpreterModelInfo::Null),
            };
        }

        None
    }
}
