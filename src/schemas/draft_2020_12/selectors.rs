use std::borrow::Cow;

use crate::utils::json_pointer::join_json_pointer;

pub trait Selectors {
    fn select_schema(&self) -> Option<&str>;
    fn select_id(&self) -> Option<&str>;
    fn select_sub_nodes(&self, node_pointer: &str) -> Vec<(String, &serde_json::Value)>;

    fn select_sub_node_def_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>>;
    fn select_sub_node_property_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>>;
    fn select_sub_node_additional_properties_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>>;
    fn select_sub_node_prefix_items_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>>;
    fn select_sub_node_items_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>>;
    fn select_sub_node_all_of_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>>;
    fn select_sub_node_any_of_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>>;
    fn select_sub_node_one_of_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>>;
}

impl Selectors for serde_json::Value {
    fn select_schema(&self) -> Option<&str> {
        self.as_object()?.get("$schema")?.as_str()
    }

    fn select_id(&self) -> Option<&str> {
        self.as_object()?.get("$id")?.as_str()
    }

    fn select_sub_nodes(&self, node_pointer: &str) -> Vec<(String, &serde_json::Value)> {
        vec![
            self.select_sub_node_def_entries(node_pointer)
                .unwrap_or_default(),
            self.select_sub_node_property_entries(node_pointer)
                .unwrap_or_default(),
            self.select_sub_node_additional_properties_entries(node_pointer)
                .unwrap_or_default(),
            self.select_sub_node_prefix_items_entries(node_pointer)
                .unwrap_or_default(),
            self.select_sub_node_items_entries(node_pointer)
                .unwrap_or_default(),
            self.select_sub_node_all_of_entries(node_pointer)
                .unwrap_or_default(),
            self.select_sub_node_any_of_entries(node_pointer)
                .unwrap_or_default(),
            self.select_sub_node_one_of_entries(node_pointer)
                .unwrap_or_default(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    //

    fn select_sub_node_def_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>> {
        let select_name = "$defs";
        let selected = self.as_object()?.get(select_name)?;

        let result = selected
            .as_object()?
            .iter()
            .map(|(sub_pointer, sub_node)| {
                (
                    join_json_pointer(vec![node_pointer, select_name, sub_pointer.as_str()]),
                    sub_node,
                )
            })
            .collect();

        Some(result)
    }
    fn select_sub_node_property_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>> {
        let select_name = "properties";
        let selected = self.as_object()?.get(select_name)?;

        let result = selected
            .as_object()?
            .iter()
            .map(|(sub_pointer, sub_node)| {
                (
                    join_json_pointer(vec![node_pointer, select_name, sub_pointer.as_str()]),
                    sub_node,
                )
            })
            .collect();

        Some(result)
    }
    fn select_sub_node_additional_properties_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>> {
        let select_name = "additionalProperties";
        let selected = self.as_object()?.get(select_name)?;

        let result = vec![(join_json_pointer(vec![node_pointer, select_name]), selected)];

        Some(result)
    }
    fn select_sub_node_prefix_items_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>> {
        let select_name = "prefixItems";
        let selected = self.as_object()?.get(select_name)?;

        let result = selected
            .as_array()?
            .iter()
            .enumerate()
            .map(|(sub_pointer, sub_node)| {
                (
                    join_json_pointer(vec![
                        node_pointer,
                        select_name,
                        sub_pointer.to_string().as_str(),
                    ]),
                    sub_node,
                )
            })
            .collect();

        Some(result)
    }
    fn select_sub_node_items_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>> {
        let select_name = "items";
        let selected = self.as_object()?.get(select_name)?;

        let result = vec![(join_json_pointer(vec![node_pointer, select_name]), selected)];

        Some(result)
    }
    fn select_sub_node_all_of_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>> {
        let select_name = "allOf";
        let selected = self.as_object()?.get(select_name)?;

        let result = selected
            .as_array()?
            .iter()
            .enumerate()
            .map(|(sub_pointer, sub_node)| {
                (
                    join_json_pointer(vec![
                        node_pointer,
                        select_name,
                        sub_pointer.to_string().as_str(),
                    ]),
                    sub_node,
                )
            })
            .collect();

        Some(result)
    }
    fn select_sub_node_any_of_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>> {
        let select_name = "anyOf";
        let selected = self.as_object()?.get(select_name)?;

        let result = selected
            .as_array()?
            .iter()
            .enumerate()
            .map(|(sub_pointer, sub_node)| {
                (
                    join_json_pointer(vec![
                        node_pointer,
                        select_name,
                        sub_pointer.to_string().as_str(),
                    ]),
                    sub_node,
                )
            })
            .collect();

        Some(result)
    }
    fn select_sub_node_one_of_entries(
        &self,
        node_pointer: &str,
    ) -> Option<Vec<(String, &serde_json::Value)>> {
        let select_name = "oneOf";
        let selected = self.as_object()?.get(select_name)?;

        let result = selected
            .as_array()?
            .iter()
            .enumerate()
            .map(|(sub_pointer, sub_node)| {
                (
                    join_json_pointer(vec![
                        node_pointer,
                        select_name,
                        sub_pointer.to_string().as_str(),
                    ]),
                    sub_node,
                )
            })
            .collect();

        Some(result)
    }

    //
}
