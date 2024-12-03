use crate::{
    expressions::{DeclarationKind, Expression},
    utils::{FoldStr, StringChecks},
};

pub struct ExpressionParser<'a> {
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub text: Vec<&'a str>,
    pub errors: Vec<ExpressionParserError>,
    pub expressions: Vec<Expression>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ExpressionParserError {
    pub line: usize,
    pub message: String,
}
impl std::fmt::Display for ExpressionParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line {}: {}", self.line, self.message)
    }
}

impl ExpressionParser<'_> {
    pub fn new(text: Vec<&str>) -> ExpressionParser {
        ExpressionParser {
            start: 0,
            current: 0,
            line: 1,
            expressions: Vec::new(),
            errors: Vec::new(),
            text,
        }
    }

    pub fn parse(mut self) -> (Vec<Expression>, Vec<ExpressionParserError>) {
        self.push_expression(Expression::Begin);
        while !self.is_at_end() {
            self.start = self.current;
            self.parse_expression();
        }
        self.push_expression(Expression::EndOfFile);
        (self.expressions, self.errors)
    }

    fn parse_expression(&mut self) {
        let c = self.advance();

        if c.is_newline() {
            self.parse_newline();
            return;
        }

        if c == ">" && self.peek() == "[" {
            self.parse_declaration();
            return;
        }

        if c == "#" {
            self.parse_header();
            return;
        }

        self.parse_paragraph();
    }

    fn parse_declaration(&mut self) {
        while !self.is_at_end() && self.peek() != "]" {
            self.advance();
        }
        let kind = self.text[self.start + 2..self.current].fold();
        self.advance();
        let text_start = self.current;
        while !self.is_at_end() && !self.peek().is_newline() {
            self.advance();
        }
        let text = self.text[text_start..self.current]
            .fold()
            .trim()
            .to_string();
        let kind = match DeclarationKind::from_str(&kind) {
            Ok(kind) => kind,
            Err(e) => {
                self.push_error(e.to_string());
                return;
            }
        };
        self.push_expression(Expression::Declaration { kind, text });
    }

    fn parse_header(&mut self) {
        while !self.is_at_end() && !self.peek().is_newline() {
            self.advance();
        }
        let text = self.text[self.start + 1..self.current]
            .fold()
            .trim()
            .to_string();
        self.push_expression(Expression::Header(text));
    }

    fn parse_paragraph(&mut self) {
        while !self.is_at_end() && !self.peek().is_newline() {
            self.advance();
        }
        let text = self.text[self.start..self.current]
            .fold()
            .trim()
            .to_string();
        self.push_expression(Expression::ParagraphLine(text));
    }

    fn parse_newline(&mut self) {
        self.push_expression(Expression::NewLine);
        self.line += 1;
    }
}

impl<'a> ExpressionParser<'a> {
    fn advance<'b>(&mut self) -> &'b str
    where
        'a: 'b,
    {
        let result = self.text[self.current];
        self.current += 1;
        result
    }
    fn peek(&self) -> &str {
        self.text[self.current]
    }
    fn is_at_end(&self) -> bool {
        self.current == self.text.len()
    }
    fn push_error(&mut self, message: String) {
        self.errors.push(ExpressionParserError {
            line: self.line,
            message,
        });
    }
    fn push_expression(&mut self, kind: Expression) {
        self.expressions.push(kind);
    }
}
