#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    StartTag {
        name: String,
        attributes: Vec<Attribute>,
        self_closing: bool
    },
    EngTag {
        name: String,
    },
    Text(String),
    Eof,
}