#[derive(Debug)]
pub enum Expressions {
    Declaration { kind: String, text: String },
    Header(String),
    ParagraphLine(String),
    Begin,
    NewLine,
    EndOfFile,
}
