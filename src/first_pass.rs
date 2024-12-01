use crate::{
    expressions::{DeclarationKind, Expression},
    utils::{FoldStr, StringChecks},
};

pub struct FirstPass<'a> {
    pub start: usize,
    pub current: usize,
    pub text: Vec<&'a str>,
    pub expressions: Vec<Expression>,
    pub line: usize,
    pub errors: Vec<FirstPassError>,
}

pub struct FirstPassError {
    pub line: usize,
    pub message: String,
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
    pub fn new(text: Vec<&str>) -> FirstPass {
        FirstPass {
            start: 0,
            current: 0,
            text,
            expressions: Vec::new(),
            line: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Vec<Expression> {
        self.expressions.push(Expression::Begin);
        while !self.is_at_end() {
            self.start = self.current;
            self.parse_expression();
        }
        self.expressions.push(Expression::EndOfFile);
        self.expressions
    }
    fn parse_expression(&mut self) {
        let c = self.advance();

        if c.is_newline() {
            self.expressions.push(Expression::NewLine);
            self.line += 1;
            return;
        }

        if c == ">" && self.peek() == "[" {
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
                    self.errors.push(FirstPassError {
                        line: self.line,
                        message: e.to_string(),
                    });
                    return;
                }
            };
            self.expressions
                .push(Expression::Declaration { kind, text });
            return;
        }

        if c == "#" {
            while !self.is_at_end() && !self.peek().is_newline() {
                self.advance();
            }
            let text = self.text[self.start + 1..self.current]
                .fold()
                .trim()
                .to_string();
            self.expressions.push(Expression::Header(text));
            return;
        }

        while !self.is_at_end() && !self.peek().is_newline() {
            self.advance();
        }

        let text = self.text[self.start..self.current]
            .fold()
            .trim()
            .to_string();
        self.expressions.push(Expression::ParagraphLine(text));
    }
}
