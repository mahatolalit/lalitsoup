use super::token::Token;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenizerState {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
}

pub struct Tokenizer<'a> {
    input: std::str::Chars<'a>,
    state: TokenizerState,
    text_buffer: String,
    current_tag_name: String,
    is_end_tag: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars(),
            state: TokenizerState::Data,
            text_buffer: String::new(),
            current_tag_name: String::new(),
            is_end_tag: false,
        }
    }

    //Add iterator
}