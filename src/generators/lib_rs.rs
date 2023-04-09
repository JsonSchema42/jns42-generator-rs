use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use rust_format::Formatter;

pub struct LibRsGenerator;

impl LibRsGenerator {
    pub fn new() -> Self {
        Self {}
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
            pub mod models;
            pub mod validators;
        });

        Ok(tokens)
    }
}
