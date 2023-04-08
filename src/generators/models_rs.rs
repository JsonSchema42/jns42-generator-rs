use crate::{schemas::LoaderContext, utils::Namer};
use quote::{format_ident, quote, TokenStreamExt};
use rust_format::Formatter;
use url::Url;

pub struct ModelsRsGenerator<'a> {
    loader_context: &'a LoaderContext<'a>,
    namer: &'a Namer<Url>,
}

impl<'a> ModelsRsGenerator<'a> {
    pub fn new(loader_context: &'a LoaderContext<'a>, namer: &'a Namer<Url>) -> Self {
        Self {
            loader_context,
            namer,
        }
    }

    pub fn generate_file_content(&self) -> Result<String, &'static str> {
        let mut tokens = quote! {};

        for node_url in self.loader_context.get_all_node_urls() {
            let name = self
                .namer
                .get_name(&node_url)
                .ok_or("could not find name")?;

            let name = name.join("_");
            let name = inflector::cases::classcase::to_class_case(&name);
            let name = format_ident!("{}", name);

            tokens.append_all(quote! {
                struct #name{

                }
            });
        }

        let formatter = rust_format::RustFmt::new();

        let content = formatter
            .format_tokens(tokens)
            .or(Err("error while formatting tokens"))?;

        Ok(content)
    }
}
