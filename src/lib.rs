pub mod types;
pub mod input;
pub mod tokenizer;
pub mod extractor;

pub use types::*;

pub fn parse(html: &str, base_url: &str) -> PageData {
    let preprocessed = input::preprocess::preprocess_html(html);
    let mut extractor = extractor::handler::Extractor::new(base_url);
    let mut tokenizer = tokenizer::state_machine::Tokenizer::new(&preprocessed);

    while let Some(token) = tokenizer.next() {
        extractor.handle_token(token);

    }
    extractor.finish()
}