use crate::{
    expressions::Expressions,
    utils::{FoldStr, StringChecks},
};

pub struct Parser<'a> {
    pub start: usize,
    pub current: usize,
    pub text: Vec<&'a str>,
    pub expressions: Vec<Expressions>,
}

impl<'a> Parser<'a> {
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
    pub fn new(text: Vec<&str>) -> Parser {
        Parser {
            start: 0,
            current: 0,
            text,
            expressions: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Vec<Expressions> {
        self.expressions.push(Expressions::Begin);
        while !self.is_at_end() {
            self.start = self.current;
            self.parse_expression();
        }
        self.expressions.push(Expressions::EndOfFile);
        self.expressions
    }
    fn parse_expression(&mut self) {
        let c = self.advance();

        if c.is_newline() {
            self.expressions.push(Expressions::NewLine);
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
            let text = self.text[text_start..self.current].fold();
            self.expressions
                .push(Expressions::Declaration { kind, text });
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
            self.expressions.push(Expressions::Header(text));
        }

        while !self.is_at_end() && !self.peek().is_newline() {
            self.advance();
        }
        let text = self.text[self.start..self.current]
            .fold()
            .trim()
            .to_string();
        self.expressions.push(Expressions::ParagraphLine(text));
    }
}
