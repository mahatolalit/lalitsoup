use super::token::{Token, Attribute};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenizerState {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
    BeforeAttrName, 
    AttrName, 
    BeforeAttrValue, 
    AttrValueDoubleQuoted, 
    AttrValueSingleQuoted, 
    AttrValueUnquoted,
}

pub struct Tokenizer<'a> {
    input: std::str::Chars<'a>,
    state: TokenizerState,
    text_buffer: String,
    current_tag_name: String,
    current_attributes: Vec<Attribute>,
    is_end_tag: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars(),
            state: TokenizerState::Data,
            text_buffer: String::new(),
            current_tag_name: String::new(),
            current_attributes: Vec::new(),
            is_end_tag: false,
        }
    }

    //Add iterator
}