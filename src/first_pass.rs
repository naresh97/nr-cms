use crate::{
    expressions::{DeclarationKind, Expression, ExpressionKind, Location},
    utils::{FoldStr, StringChecks},
};

pub struct FirstPass<'a> {
    pub file: String,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub text: Vec<&'a str>,
    pub errors: Vec<FirstPassError>,
    pub expressions: Vec<Expression>,
}

pub struct FirstPassError {
    pub line: usize,
    pub message: String,
}

impl FirstPass<'_> {
    pub fn new(text: Vec<&str>, file: String) -> FirstPass {
        FirstPass {
            start: 0,
            current: 0,
            line: 0,
            expressions: Vec::new(),
            errors: Vec::new(),
            text,
            file,
        }
    }

    pub fn parse(mut self) -> Vec<Expression> {
        self.push_expression(ExpressionKind::Begin);
        while !self.is_at_end() {
            self.start = self.current;
            self.parse_expression();
        }
        self.push_expression(ExpressionKind::EndOfFile);
        self.expressions
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
        self.push_expression(ExpressionKind::Declaration { kind, text });
    }

    fn parse_header(&mut self) {
        while !self.is_at_end() && !self.peek().is_newline() {
            self.advance();
        }
        let text = self.text[self.start + 1..self.current]
            .fold()
            .trim()
            .to_string();
        self.push_expression(ExpressionKind::Header(text));
    }

    fn parse_paragraph(&mut self) {
        while !self.is_at_end() && !self.peek().is_newline() {
            self.advance();
        }
        let text = self.text[self.start..self.current]
            .fold()
            .trim()
            .to_string();
        self.push_expression(ExpressionKind::ParagraphLine(text));
    }

    fn parse_newline(&mut self) {
        self.push_expression(ExpressionKind::NewLine);
        self.line += 1;
    }
}

impl<'a> FirstPass<'a> {
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
    fn peek_next(&self) -> &str {
        self.text[self.current + 1]
    }
    fn is_at_end(&self) -> bool {
        self.current == self.text.len()
    }

    fn push_error(&mut self, message: String) {
        self.errors.push(FirstPassError {
            line: self.line,
            message,
        });
    }

    fn push_expression(&mut self, kind: ExpressionKind) {
        self.expressions.push(Expression {
            kind,
            location: Location {
                line: self.line,
                file: self.file.clone(),
            },
        });
    }
}
