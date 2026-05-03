use crate::types::PageData;
use crate::tokenizer::token::Token;
pub struct Extractor {
    data: PageData,
}

impl Extractor {
    pub fn new(base_url: &str) -> Self {
        let mut data = PageData::default();
        data.url = base_url.to_string();
        Self {
            data
        }
    }

    pub fn handle_token(&mut self, token: Token) {
        if let Token::Text(text) = token {
            self.data.body_text.push_str(&text);
            self.data.body_text.push(' ');
        }
    }
    pub fn finish(self) -> PageData {
        self.data
    }
}