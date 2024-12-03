use anyhow::bail;

use crate::expressions::{DeclarationKind, Expression};

pub struct SecondPass {
    current: usize,
    expressions: Vec<Expression>,
    output_site: Site,
}

impl SecondPass {
    pub fn new(expressions: Vec<Expression>) -> SecondPass {
        SecondPass {
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
                Expression::NewLine => expressions.push(c.clone()),
                Expression::Header(_) | Expression::ParagraphLine(_) => {
                    let mut c = c.clone();
                    update_links_in_expression(&mut c);
                    expressions.push(c);
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
}

fn generate_page_filename(name: &str) -> String {
    let name = name.trim().to_ascii_lowercase().replace(" ", "-");
    let name = format!("{}.html", name);
    name
}

fn update_links_in_expression(expression: &mut Expression) {
    let text = match expression {
        Expression::Header(text) => text,
        Expression::ParagraphLine(text) => text,
        _ => return,
    };
    let mut current = 0;
    let t = text.chars().collect::<Vec<_>>();
    let mut new_t = Vec::with_capacity(t.len());
    while current != t.len() {
        let c = t[current];
        let peek = t.get(current + 1);
        if matches!(c, '[') && matches!(peek, Some('[')) {
            let start = current + 2;
            while current != t.len() {
                let c = t[current];
                let peek = t.get(current + 1);
                if matches!(c, ']') && matches!(peek, Some(']')) {
                    let page_name = t[start..current].iter().fold(String::new(), |mut a, b| {
                        let mut buf = [0u8; 4];
                        let b = b.encode_utf8(&mut buf);
                        a += b;
                        a
                    });
                    let filename = generate_page_filename(&page_name);
                    let text = format!("<a href=\"{}\">{}</a>", filename, page_name);
                    let mut text = text.chars().collect::<Vec<_>>();
                    new_t.append(&mut text);
                    current += 2;
                    break;
                }
                current += 1;
            }
        } else {
            new_t.push(c);
            current += 1;
        }
    }
    let new_t = new_t.iter().fold(String::new(), |mut a, b| {
        let mut buf = [0u8; 4];
        let b = b.encode_utf8(&mut buf);
        a += b;
        a
    });
    *text = new_t;
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
