use std::collections::HashMap;

use crate::expressions::{DeclarationKind, Expression};

pub struct SecondPass {
    current: usize,
    expressions: Vec<Expression>,
}

impl SecondPass {
    fn advance(&mut self) -> &Expression {
        let result = &self.expressions[self.current];
        self.current += 1;
        result
    }
    fn peek(&self) -> &Expression {
        &self.expressions[self.current]
    }
    fn is_at_end(&self) -> bool {
        self.current == self.expressions.len()
            || matches!(self.expressions[self.current], Expression::EndOfFile)
    }
    pub fn new(expressions: Vec<Expression>) -> SecondPass {
        SecondPass {
            current: 0,
            expressions,
        }
    }
    pub fn parse(mut self) -> Site {
        while !self.is_at_end() {
            let c = self.advance();

            if matches!(c, Expression::Begin) {
                continue;
            }

            if let Expression::Declaration {
                kind: DeclarationKind::SiteTitle,
                text: title,
            } = c
            {
                let title = title.to_string();
                let site = self.parse_site(title);
                return site;
            }
        }
        panic!("Site not found!");
    }
    fn parse_site(&mut self, name: String) -> Site {
        let mut pages = Vec::new();
        while !self.is_at_end() {
            let c = self.advance();
            if let Expression::Declaration {
                kind: DeclarationKind::Page,
                text: page_name,
            } = c
            {
                let page_name = page_name.clone();
                let page = self.parse_page(page_name);
                pages.push(page);
            }
        }
        Site {
            name,
            pages,
            page_directory: Default::default(),
        }
    }

    fn parse_page(&mut self, name: String) -> Page {
        let mut expressions = Vec::new();
        while !self.is_at_end() {
            let c = self.peek();
            match c {
                Expression::Header(_) | Expression::ParagraphLine(_) | Expression::NewLine => {
                    expressions.push(c.clone())
                }
                _ => break,
            };
            self.advance();
        }
        Page {
            name,
            expressions,
            filename: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct Site {
    pub name: String,
    pub pages: Vec<Page>,
    pub page_directory: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Page {
    pub name: String,
    pub filename: String,
    pub expressions: Vec<Expression>,
}
