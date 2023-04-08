use quote::{quote, TokenStreamExt, __private::TokenStream};
use rust_format::Formatter;

pub struct LibRsGenerator;

impl LibRsGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_file_content(&self) -> Result<String, &'static str> {
        let tokens = self.generate_file_token_stream()?;

        let formatter = rust_format::RustFmt::new();

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
