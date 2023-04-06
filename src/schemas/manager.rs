use super::{loader::Loader, meta::MetaSchemaId};
use crate::schemas;
use std::{cell::RefCell, collections::HashMap, fs::File, rc::Rc};
use url::Url;

#[derive(Default)]
pub struct Manager<'a> {
    loaders: HashMap<MetaSchemaId, Box<dyn Loader<'a> + 'a>>,
    retrieval_root_node_map: HashMap<Url, Url>,
    root_node_retrieval_map: HashMap<Url, Url>,
    root_node_meta_schema_id_map: HashMap<Url, MetaSchemaId>,
}

impl<'a> Manager<'a> {
    pub fn new() -> Rc<RefCell<Self>> {
        let manager = Self::default();

        let manager = RefCell::new(manager);
        let manager = Rc::new(manager);

        manager.borrow_mut().add_loader(
            MetaSchemaId::Draft202012,
            Box::new(schemas::draft_2020_12::loader::LoaderImpl::new(
                Rc::downgrade(&manager),
            )),
        );
        manager.borrow_mut().add_loader(
            MetaSchemaId::Draft201909,
            Box::new(schemas::draft_2020_12::loader::LoaderImpl::new(
                Rc::downgrade(&manager),
            )),
        );
        manager.borrow_mut().add_loader(
            MetaSchemaId::Draft07,
            Box::new(schemas::draft_2020_12::loader::LoaderImpl::new(
                Rc::downgrade(&manager),
            )),
        );
        manager.borrow_mut().add_loader(
            MetaSchemaId::Draft06,
            Box::new(schemas::draft_2020_12::loader::LoaderImpl::new(
                Rc::downgrade(&manager),
            )),
        );
        manager.borrow_mut().add_loader(
            MetaSchemaId::Draft04,
            Box::new(schemas::draft_2020_12::loader::LoaderImpl::new(
                Rc::downgrade(&manager),
            )),
        );

        manager
    }

    pub fn load_from_root_node(
        &self,
        node: &serde_json::Value,
        node_url: &'a Url,
        retrieval_url: &'a Url,
        referencing_url: Option<&'a Url>,
        default_meta_schema_id: MetaSchemaId,
    ) -> Result<Url, &'static str> {
        let mut schema_id = self.discover_schema_id(node);
        if schema_id == MetaSchemaId::Unknown {
            schema_id = default_meta_schema_id;
        }

        let loader = self.loaders.get(&schema_id).unwrap();

        let node_url = loader.load_from_root_node(
            node,
            node_url,
            retrieval_url,
            referencing_url,
            default_meta_schema_id,
        )?;

        Ok(node_url)
    }

    pub fn load_from_url(
        &mut self,
        node_url: &'a Url,
        retrieval_url: &'a Url,
        referencing_url: Option<&'a Url>,
        default_meta_schema_id: MetaSchemaId,
    ) -> Result<Url, &'static str> {
        if let Some(root_node_url) = self.retrieval_root_node_map.get(retrieval_url) {
            return Ok(root_node_url.clone());
        }

        let root_node = self.fetch_root_node_from_url(retrieval_url)?;

        let root_node_url = self.load_from_root_node(
            &root_node,
            node_url,
            retrieval_url,
            referencing_url,
            default_meta_schema_id,
        )?;

        self.retrieval_root_node_map
            .insert(retrieval_url.clone(), root_node_url.clone());
        self.root_node_retrieval_map
            .insert(root_node_url.clone(), retrieval_url.clone());

        Ok(root_node_url.clone())
    }

    pub fn fetch_root_node_from_url(
        &self,
        url: &'a Url,
    ) -> Result<serde_json::Value, &'static str> {
        match url.scheme() {
            "file" => {
                let path = url.path();
                let reader = File::open(path).or(Err("error reading file"))?;

                let value: serde_json::Value =
                    serde_json::from_reader(reader).or(Err("error deserializing file content"))?;

                Ok(value)
            }
            _ => Err("not supported"),
        }
    }

    pub fn discover_schema_id(&self, node: &serde_json::Value) -> MetaSchemaId {
        for (schema_id, loader) in self.loaders.iter() {
            if loader.is_schema_root_node(node) {
                return *schema_id;
            }
        }

        MetaSchemaId::Unknown
    }

    fn add_loader(&mut self, meta_schema_id: MetaSchemaId, loader: Box<dyn Loader<'a> + 'a>) {
        self.loaders.insert(meta_schema_id, loader);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_manager() {
        let _manager = Manager::new();
    }
}
