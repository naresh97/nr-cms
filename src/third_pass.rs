use std::collections::HashMap;

use crate::{expressions::Expression, second_pass::Site};

pub struct ThirdPass {
    site: Site,
}

impl ThirdPass {
    pub fn new(site: Site) -> Self {
        ThirdPass { site }
    }
    pub fn parse(mut self) -> Site {
        let page_directory = self.generate_page_directory();
        self.site.page_directory = page_directory;
        self.update_page_filenames();
        self.update_links_in_site();
        self.site
    }

    fn generate_page_directory(&self) -> HashMap<String, String> {
        let mut page_directory = HashMap::new();
        for page in &self.site.pages {
            let filename = generate_page_filename(&page.name);
            page_directory.insert(page.name.clone(), filename);
        }
        page_directory
    }

    fn update_page_filenames(&mut self) {
        for page in &mut self.site.pages {
            page.filename = self
                .site
                .page_directory
                .get(&page.name)
                .unwrap()
                .to_string();
        }
    }

    fn update_links_in_site(&mut self) {
        for page in &mut self.site.pages {
            for expression in &mut page.expressions {
                update_links_in_expression(expression, &self.site.page_directory);
            }
        }
    }
}

fn update_links_in_expression(
    expression: &mut Expression,
    page_directory: &HashMap<String, String>,
) {
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
                    let filename = page_directory.get(&page_name).unwrap();
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

fn generate_page_filename(name: &str) -> String {
    let name = name.trim().to_ascii_lowercase().replace(" ", "-");
    let name = format!("{}.html", name);
    name
}
