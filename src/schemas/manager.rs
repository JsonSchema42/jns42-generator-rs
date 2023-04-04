use super::{loader::Loader, meta::MetaSchemaId};
use crate::schemas;
use std::{borrow::Cow, cell::RefCell, collections::HashMap, fs::File, rc::Rc};
use url::Url;

#[derive(Default)]
pub struct Manager<'a> {
    loaders: HashMap<MetaSchemaId, Box<dyn Loader<'a> + 'a>>,
    retrieval_root_node_map: HashMap<Url, Url>,
    root_node_retrieval_map: HashMap<Url, Url>,
    root_nodes: HashMap<Url, serde_json::Value>,
}

impl<'a> Manager<'a> {
    pub fn new() -> Rc<RefCell<Option<Self>>> {
        let manager = None;
        let manager = RefCell::new(manager);
        let manager = Rc::new(manager);

        let mut loaders: HashMap<MetaSchemaId, Box<dyn Loader<'a>>> = HashMap::new();

        loaders.insert(
            MetaSchemaId::Draft202012,
            Box::new(schemas::draft_2020_12::loader::LoaderImpl::new(
                Rc::downgrade(&manager),
            )),
        );
        loaders.insert(
            MetaSchemaId::Draft201909,
            Box::new(schemas::draft_2020_12::loader::LoaderImpl::new(
                Rc::downgrade(&manager),
            )),
        );
        loaders.insert(
            MetaSchemaId::Draft07,
            Box::new(schemas::draft_2020_12::loader::LoaderImpl::new(
                Rc::downgrade(&manager),
            )),
        );
        loaders.insert(
            MetaSchemaId::Draft06,
            Box::new(schemas::draft_2020_12::loader::LoaderImpl::new(
                Rc::downgrade(&manager),
            )),
        );
        loaders.insert(
            MetaSchemaId::Draft04,
            Box::new(schemas::draft_2020_12::loader::LoaderImpl::new(
                Rc::downgrade(&manager),
            )),
        );

        *manager.borrow_mut() = Some(Manager {
            loaders,
            ..Default::default()
        });

        manager
    }

    pub fn load_from_root_node(
        &mut self,
        node: &'a serde_json::Value,
        node_url: &'a Url,
        retrieval_url: &'a Url,
        referencing_url: Option<&'a Url>,
        default_meta_schema_id: MetaSchemaId,
    ) -> Result<Cow<'a, Url>, &'static str> {
        let mut schema_id = self.discover_schema_id(node);
        if schema_id == MetaSchemaId::Unknown {
            schema_id = default_meta_schema_id;
        }

        let loader = self.loaders.get_mut(&schema_id).unwrap();

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
    ) -> Result<Cow<'a, Url>, &'static str> {
        if let Some(root_node_url) = self.retrieval_root_node_map.get(retrieval_url) {
            return Ok(Cow::Owned(root_node_url.clone()));
        }

        let root_node = self.fetch_root_node_from_url(retrieval_url)?;
        self.root_nodes.insert(retrieval_url.clone(), root_node);

        let root_node = self.root_nodes.get(retrieval_url).unwrap();

        todo!();

        let root_node_url_cow = self.load_from_root_node(
            root_node,
            node_url,
            retrieval_url,
            referencing_url,
            default_meta_schema_id,
        )?;

        let root_node_url = root_node_url_cow.into_owned();

        self.retrieval_root_node_map
            .insert(retrieval_url.clone(), root_node_url.clone());
        self.root_node_retrieval_map
            .insert(root_node_url.clone(), retrieval_url.clone());

        let root_node_url_cow = Cow::Owned(root_node_url);

        Ok(root_node_url_cow)
    }

    pub fn fetch_root_node_from_url(&self, url: &Url) -> Result<serde_json::Value, &'static str> {
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
}
