use std::collections::HashMap;

use crate::{args, types::cms_page::CMSPage};

use super::template_generators::*;

pub fn gen_pages(
    pages: &HashMap<String, CMSPage>,
    generation_dirs: &args::GenerationDirs,
) -> String {
    let mut pages_string = String::new();
    for (name, page) in pages {
        let templates = &page.templates;
        let paragraphs = gen_paragraphs(templates);
        let links = gen_links(templates);
        let image = gen_image(templates, generation_dirs);
        let blog = gen_blog(templates);

        let page_string = format!(
            r#"
        <div id="page-{name}" class="page">
        {image}
        {paragraphs}
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

    use crate::{
        args::GenerationDirs,
        types::{cms_page::CMSPage, template_type::TemplateType},
    };

    use super::gen_pages;

    #[test]
    fn test_gen_pages() {
        let pages = HashMap::from([(
            "FirstPage".to_string(),
            CMSPage {
                templates: Vec::from([TemplateType::Name {
                    name: "FirstPage".to_string(),
                }]),
            },
        )]);
        let generation_dirs = GenerationDirs {
            generation_dir: Default::default(),
            source_dir: Default::default(),
        };
        let gen = gen_pages(&pages, &generation_dirs);
        assert!(gen.contains("FirstPage"));
    }
}
