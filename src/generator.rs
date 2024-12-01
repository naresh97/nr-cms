use crate::{
    expressions::Expression,
    second_pass::{Page, Site},
};

pub struct Generator {
    site: Site,
}
impl Generator {
    pub fn new(site: Site) -> Generator {
        Generator { site }
    }
    pub fn generate(&self) -> Vec<File> {
        let pages = self
            .site
            .pages
            .iter()
            .map(Self::generate_page)
            .collect::<Vec<_>>();
        pages
    }
    fn generate_page(page: &Page) -> File {
        let mut body = String::new();
        let mut current = 0;
        while current != page.expressions.len() {
            let c = &page.expressions[current];

            if let Expression::Header(header) = c {
                body += &format!("<h1>{}</h1>\n", header);
                current += 1;
                continue;
            }

            if let Expression::ParagraphLine(line) = c {
                let mut paragraph = line.to_string();
                current += 1;
                while current != page.expressions.len() {
                    let c = &page.expressions[current];
                    if let Expression::ParagraphLine(line) = c {
                        paragraph += &format!(" {}", line);
                        current += 1;
                        continue;
                    }

                    let peek = page.expressions.get(current + 1);

                    if matches!(c, Expression::NewLine) && matches!(peek, Some(Expression::NewLine))
                    {
                        current += 2;
                        break;
                    } else if matches!(c, Expression::NewLine) {
                        current += 1;
                        continue;
                    }

                    break;
                }
                body += &format!("<p>{}</p>", paragraph);
                continue;
            }
            if matches!(c, Expression::NewLine | Expression::EndOfFile) {
                current += 1;
                continue;
            }
        }
        File {
            name: page.name.clone(),
            content: body,
        }
    }
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub content: String,
}
