use anyhow::bail;

#[derive(Debug, Clone)]
pub enum Expression {
    Declaration { kind: DeclarationKind, text: String },
    Header(String),
    ParagraphLine(String),
    Begin,
    NewLine,
    EndOfFile,
}

#[derive(Debug, Clone)]
pub enum DeclarationKind {
    SiteTitle,
    Page,
}

impl DeclarationKind {
    pub fn from_str(text: &str) -> anyhow::Result<DeclarationKind> {
        match text {
            "Site-Title" => Ok(DeclarationKind::SiteTitle),
            "Page" => Ok(DeclarationKind::Page),
            _ => bail!("{} is not a valid declartion type", text),
        }
    }
}
