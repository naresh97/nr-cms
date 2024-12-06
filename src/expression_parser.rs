use std::path::{Path, PathBuf};

use crate::expressions::{DeclarationKind, Expression};

pub struct ExpressionParser {
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub text: Vec<char>,
    pub errors: Vec<Error>,
    pub expressions: Vec<Expression>,
    pub filename: PathBuf,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Error {
    pub line: usize,
    pub message: String,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line {}: {}", self.line, self.message)
    }
}

impl ExpressionParser {
    pub fn new(text: Vec<char>, filename: &Path) -> ExpressionParser {
        ExpressionParser {
            start: 0,
            current: 0,
            line: 1,
            expressions: Vec::new(),
            errors: Vec::new(),
            text,
            filename: filename.to_path_buf(),
        }
    }
    pub fn from_file(filename: &Path) -> anyhow::Result<ExpressionParser> {
        let text = std::fs::read_to_string(filename)?;
        let text = text.chars().collect::<Vec<_>>();
        Ok(Self::new(text, filename))
    }

    pub fn parse(mut self) -> (Vec<Expression>, Vec<Error>) {
        self.push_expression(Expression::Begin);
        while !self.is_at_end() {
            self.start = self.current;
            self.parse_expression();
        }
        self.push_expression(Expression::EndOfFile);
        (self.expressions, self.errors)
    }

    fn parse_expression(&mut self) {
        if self.advance_if_newline() {
            self.parse_newline();
            return;
        }

        let c = self.advance();

        if c == '>' && self.peek() == '[' {
            self.parse_declaration();
            return;
        }

        if c == '#' {
            self.parse_header();
            return;
        }

        self.parse_paragraph();
    }

    fn parse_declaration(&mut self) {
        while !self.is_at_end() && self.peek() != ']' {
            self.advance();
        }
        let kind = self.text[self.start + 2..self.current]
            .iter()
            .collect::<String>();
        self.advance();
        let text_start = self.current;
        while !self.is_at_end() && !self.peek_newline() {
            self.advance();
        }
        let text = self.text[text_start..self.current]
            .iter()
            .collect::<String>()
            .trim()
            .to_string();
        let kind = match DeclarationKind::from_str(&kind) {
            Ok(kind) => kind,
            Err(e) => {
                self.push_error(e.to_string());
                return;
            }
        };
        if matches!(kind, DeclarationKind::Include) {
            self.process_include(&text);
            return;
        }
        self.push_expression(Expression::Declaration { kind, text });
    }

    fn parse_header(&mut self) {
        while !self.is_at_end() && !self.peek_newline() {
            self.advance();
        }
        let text = self.text[self.start + 1..self.current]
            .iter()
            .collect::<String>()
            .trim()
            .to_string();
        self.push_expression(Expression::Header(text));
    }

    fn parse_paragraph(&mut self) {
        while !self.is_at_end() && !self.peek_newline() {
            self.advance();
        }
        let text = self.text[self.start..self.current]
            .iter()
            .collect::<String>()
            .trim()
            .to_string();
        self.push_expression(Expression::ParagraphLine(text));
    }

    fn parse_newline(&mut self) {
        self.push_expression(Expression::NewLine);
        self.line += 1;
    }

    fn process_include(&mut self, include_file: &str) {
        let parent = self.filename.parent().unwrap_or(Path::new("."));
        let include_file = parent.join(Path::new(include_file));

        let Ok(expression_parser) = Self::from_file(&include_file) else {
            self.push_error(format!("Can't open file: {include_file:?}"));
            return;
        };
        let (mut expressions, errors) = expression_parser.parse();
        for e in errors {
            println!("Expression error in file {include_file:?}: {e}");
        }
        self.expressions.append(&mut expressions);
    }
}

impl ExpressionParser {
    fn advance(&mut self) -> char {
        let result = self.text[self.current];
        self.current += 1;
        result
    }
    fn peek(&self) -> char {
        self.text[self.current]
    }
    fn is_at_end(&self) -> bool {
        self.current == self.text.len()
    }
    fn advance_if_newline(&mut self) -> bool {
        let newline_windows = "\r\n".chars();
        let is_windows = self
            .text
            .get(self.current..self.current + 2)
            .is_some_and(|c| c.iter().copied().eq(newline_windows));
        let peek = self.text[self.current];
        if is_windows {
            self.advance();
            self.advance();
            true
        } else if peek == '\r' || peek == '\n' {
            self.advance();
            true
        } else {
            false
        }
    }
    fn peek_newline(&self) -> bool {
        let newline_windows = "\r\n".chars();
        let is_windows = self
            .text
            .get(self.current..self.current + 2)
            .is_some_and(|c| c.iter().copied().eq(newline_windows));
        let peek = self.text[self.current];
        is_windows || peek == '\r' || peek == '\n'
    }
    fn push_error(&mut self, message: String) {
        self.errors.push(Error {
            line: self.line,
            message,
        });
    }
    fn push_expression(&mut self, kind: Expression) {
        self.expressions.push(kind);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_newline() {
        let x = "\r\n";
        let x = x.chars();
        println!("{x:?}");
    }
}
