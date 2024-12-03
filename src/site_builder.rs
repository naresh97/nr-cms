use anyhow::bail;

use crate::{
    expressions::{DeclarationKind, Expression},
    utils::generate_page_filename,
};

pub struct SiteBuilder {
    current: usize,
    expressions: Vec<Expression>,
    output_site: Site,
}

#[derive(Debug, Default)]
pub struct Site {
    pub name: String,
    pub pages: Vec<Page>,
}

#[derive(Debug, Default)]
pub struct Page {
    pub name: String,
    pub filename: String,
    pub expressions: Vec<Expression>,
}

impl SiteBuilder {
    pub fn new(expressions: Vec<Expression>) -> SiteBuilder {
        SiteBuilder {
            current: 0,
            expressions,
            output_site: Site::default(),
        }
    }
    pub fn parse(mut self) -> anyhow::Result<Site> {
        if !matches!(self.advance(), Expression::Begin) {
            bail!("First pass was malformed. Begin expression was not found.");
        }
        while !self.is_at_end() {
            self.parse_expression();
        }
        Ok(self.output_site)
    }

    fn parse_expression(&mut self) {
        let c = self.advance();

        if let Expression::Declaration {
            kind: DeclarationKind::SiteTitle,
            text: title,
        } = c
        {
            self.output_site.name = title.to_string();
            return;
        }

        if let Expression::Declaration {
            kind: DeclarationKind::Page,
            text: page_name,
        } = c
        {
            let page_name = page_name.clone();
            let page = self.parse_page(page_name);
            self.output_site.pages.push(page);
        }
    }

    fn parse_page(&mut self, name: String) -> Page {
        let mut expressions = Vec::new();
        while !self.is_at_end() {
            let c = self.peek();
            match c {
                Expression::NewLine | Expression::Header(_) | Expression::ParagraphLine(_) => {
                    expressions.push(c.clone());
                }
                _ => break,
            };
            self.advance();
        }
        Page {
            filename: generate_page_filename(&name),
            name,
            expressions,
        }
    }
}

impl SiteBuilder {
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
}
