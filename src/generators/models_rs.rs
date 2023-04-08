use quote::{format_ident, quote, TokenStreamExt};
use rust_format::Formatter;

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

    pub fn generate_file_content(&self) -> Result<String, &'static str> {
        let mut tokens = quote! {};

        {
            let name = format_ident!("Model1");
            tokens.append_all(quote! {
                struct #name{

                }
            });
        }

        {
            let name = format_ident!("Model2");
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
