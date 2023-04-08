use crate::{schemas::LoaderContext, utils::Namer};
use quote::{format_ident, quote, TokenStreamExt, __private::TokenStream};
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
        let tokens = self.generate_file_tokenstream()?;

        let formatter = rust_format::RustFmt::new();

        let content = formatter
            .format_tokens(tokens)
            .or(Err("error while formatting tokens"))?;

        Ok(content)
    }

    fn generate_file_tokenstream(&self) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};

        tokens.append_all(quote! {
            use serde::{Deserialize, Serialize};
        });

        for node_url in self.loader_context.get_all_node_urls() {
            tokens.append_all(self.generate_model_tokenstream(&node_url));
        }

        Ok(tokens)
    }

    fn generate_model_tokenstream(&self, node_url: &Url) -> Result<TokenStream, &'static str> {
        let name = self.namer.get_name(node_url).ok_or("could not find name")?;

        let name = name.join("_");
        let name = inflector::cases::classcase::to_class_case(&name);
        let name = format_ident!("{}", name);

        let mut tokens = quote! {};

        tokens.append_all(quote! {
            #[derive(Serialize, Deserialize, Debug, Default)]
            pub struct #name{

            }
        });

        Ok(tokens)
    }
}
