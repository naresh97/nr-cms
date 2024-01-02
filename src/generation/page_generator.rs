use std::collections::HashMap;

use crate::types::{cms_page::CMSPage, generation_dirs::GenerationDirs};

use super::template_generators::*;

pub fn gen_pages(
    pages: &HashMap<String, CMSPage>,
    generation_dirs: &impl GenerationDirs,
) -> String {
    let mut pages_string = String::new();
    for (name, page) in pages {
        let templates = &page.templates;
        let order_preserved_elements = gen_order_preserved_elements(templates, generation_dirs);
        let links = gen_links(templates);
        let blog = gen_blog(templates, generation_dirs);

        let page_string = format!(
            r#"
        <div id="page-{name}" class="page">
        {order_preserved_elements}
        {links}
        {blog}
        </div>
        "#
        );
        pages_string.push_str(&page_string);
    }
    pages_string
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::types::{
        cms_page::CMSPage, generation_dirs::TempGenerationDirs, template_type::TemplateType,
    };

    use super::gen_pages;

    #[test]
    fn test_gen_pages() {
        let pages = HashMap::from([(
            "FirstPage".to_string(),
            CMSPage {
                templates: Vec::from([
                    TemplateType::Name {
                        name: "FirstPage".to_string(),
                    },
                    TemplateType::Paragraph {
                        content: "Second".to_string(),
                    },
                    TemplateType::Code {
                        code: "Third".to_string(),
                    },
                    TemplateType::Image {
                        url: "sample.jpg".to_string(),
                        copy_asset: false,
                        size: Some(10),
                    },
                ]),
            },
        )]);
        let gen = gen_pages(&pages, &TempGenerationDirs::default());
        assert!(gen.contains("FirstPage"));
        assert!(gen.contains("Second"));
        assert!(gen.contains("Third"));
    }
}
