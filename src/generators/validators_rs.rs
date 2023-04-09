use crate::{schemas::InterpreterContext, utils::Namer};
use inflector::cases::{classcase::to_class_case, snakecase::to_snake_case};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use rust_format::Formatter;
use url::Url;

pub struct ValidatorsRsGenerator<'a> {
    loader_context: &'a InterpreterContext<'a>,
    namer: &'a Namer<Url>,
}

impl<'a> ValidatorsRsGenerator<'a> {
    pub fn new(loader_context: &'a InterpreterContext<'a>, namer: &'a Namer<Url>) -> Self {
        Self {
            loader_context,
            namer,
        }
    }

    pub fn generate_file_content(&self) -> Result<String, &'static str> {
        let mut tokens = quote!();

        tokens.append_all(quote! {
            ///@generated
        });

        tokens.append_all(self.generate_file_token_stream()?);

        let configuration = rust_format::Config::new_str().edition(rust_format::Edition::Rust2021);
        let formatter = rust_format::RustFmt::from_config(configuration);

        let content = formatter
            .format_tokens(tokens)
            .or(Err("error while formatting tokens"))?;

        Ok(content)
    }

    fn generate_file_token_stream(&self) -> Result<TokenStream, &'static str> {
        let mut tokens = quote! {};

        tokens.append_all(quote! {
            use super::models;
        });

        for node_url in self.loader_context.get_all_node_urls() {
            tokens.append_all(self.generate_model_token_stream(&node_url));
        }

        Ok(tokens)
    }

    fn generate_model_token_stream(&self, node_url: &Url) -> Result<TokenStream, &'static str> {
        let node_name = self.namer.get_name(node_url).ok_or("could not find name")?;

        let validator_name = node_name.join(" ");
        let validator_name = format!("validate_{}", validator_name);
        let validator_name = to_snake_case(&validator_name);
        let validator_name = format_ident!("r#{}", validator_name);

        let model_name = node_name.join(" ");
        let model_name = to_class_case(&model_name);
        let model_name = format_ident!("r#{}", model_name);

        let mut tokens = quote! {};

        tokens.append_all(quote! {

            pub fn #validator_name(model: &models::#model_name) -> bool {
                todo!();
            }


        });

        Ok(tokens)
    }
}
