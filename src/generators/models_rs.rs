use crate::schemas::LoaderContext;

pub struct ModelsRsGenerator<'a> {
    schema_loader: &'a LoaderContext<'a>,
}

impl<'a> ModelsRsGenerator<'a> {
    pub fn new(loader: &'a LoaderContext<'a>) -> Self {
        Self {
            schema_loader: loader,
        }
    }
}
