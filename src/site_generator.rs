use crate::{
    expressions::Expression,
    site_builder::{Page, Site},
};

pub struct SiteGenerator {
    site: Site,
}
impl SiteGenerator {
    pub fn new(site: Site) -> SiteGenerator {
        SiteGenerator { site }
    }
    pub fn generate(self) -> Vec<PageBody> {
        let mut page_bodies = self
            .site
            .pages
            .iter()
            .map(Self::generate_page)
            .collect::<Vec<_>>();
        self.insert_front_matter(&mut page_bodies);
        page_bodies
    }
    fn generate_page(page: &Page) -> PageBody {
        let mut body = String::new();
        let mut current = 0;
        while current != page.expressions.len() {
            let c = &page.expressions[current];

            if let Expression::Header(header) = c {
                body += &format!("<h1>{header}</h1>\n");
                current += 1;
                continue;
            }

            if let Expression::ParagraphLine(line) = c {
                let mut paragraph = line.to_string();
                current += 1;
                while current != page.expressions.len() {
                    let c = &page.expressions[current];
                    if let Expression::ParagraphLine(line) = c {
                        paragraph += &format!(" {line}");
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
                body += &format!("<p>{paragraph}</p>");
                continue;
            }
            if matches!(c, Expression::NewLine | Expression::EndOfFile) {
                current += 1;
                continue;
            }
        }
        PageBody {
            name: page.name.clone(),
            content: body,
            filename: page.filename.clone(),
        }
    }

    fn insert_front_matter(&self, page_bodies: &mut [PageBody]) {
        let style = include_str!("../resources/default-style.css");
        let navbar = self.generate_navbar();
        for body in page_bodies {
            body.content = format!(
                "<html>
<head>
<title>{0} - {1}</title>
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
<style type=\"text/css\">
{2}
</style>
</head>
<body>
{3}
{4}
</body>
</html>",
                body.name, self.site.name, style, navbar, body.content
            );
        }
    }

    fn generate_navbar(&self) -> String {
        let links = self
            .site
            .pages
            .iter()
            .map(
                |Page {
                     name,
                     filename,
                     expressions: _,
                 }| format!("<a href=\"{filename}\">{name}</a>"),
            )
            .collect::<Vec<_>>()
            .join(" - ");
        format!("<div class=\"navbar\">{links}</div><hr/>")
    }
}

#[derive(Debug)]
pub struct PageBody {
    pub name: String,
    pub content: String,
    pub filename: String,
}
