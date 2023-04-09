use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use rust_format::Formatter;

pub fn generate_file_content(file_tokens: TokenStream) -> Result<String, &'static str> {
    let mut tokens = quote!();

    tokens.append_all(quote! {
        ///@generated
    });

    tokens.append_all(file_tokens);

    let configuration = rust_format::Config::new_str().edition(rust_format::Edition::Rust2021);
    let formatter = rust_format::RustFmt::from_config(configuration);

    let content = formatter
        .format_tokens(tokens)
        .or(Err("error while formatting tokens"))?;

    Ok(content)
}
