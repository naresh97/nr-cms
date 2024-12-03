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
    pub fn generate(self) -> Vec<PageBody> {
        let mut page_bodies = self
            .site
            .pages
            .iter()
            .map(Self::generate_page)
            .collect::<Vec<_>>();
        let navbar = self.generate_navbar();
        self.insert_front_matter(navbar, &mut page_bodies);
        page_bodies
    }
    fn generate_page(page: &Page) -> PageBody {
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
        PageBody {
            name: page.name.clone(),
            content: body,
        }
    }

    fn insert_front_matter(&self, navbar: String, page_bodies: &mut [PageBody]) {
        let style = include_str!("../resources/default-style.css");
        for body in page_bodies {
            body.content = format!(
                "<html>
<head>
<title>{} - {}</title>
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
<style type=\"text/css\">
{}
</style>
</head>
<body>
{}
{}
</body>
</html>",
                body.name, self.site.name, style, navbar, body.content
            );
        }
    }

    fn generate_navbar(&self) -> String {
        self.site
            .page_directory
            .iter()
            .map(|(name, filename)| format!("<a href=\"{}\">{}</a>", filename, name))
            .collect::<Vec<_>>()
            .join(" - ")
    }
}

#[derive(Debug)]
pub struct PageBody {
    pub name: String,
    pub content: String,
}
