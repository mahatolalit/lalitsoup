#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    StartTag {name: String},
    EngTag {name: String},
    Text(String),
    Eof,
}