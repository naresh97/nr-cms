use std::collections::HashMap;

use crate::{run_args, types::cms_page::CMSPage};

use super::template_generators::*;

pub fn gen_pages(pages: &HashMap<String, CMSPage>, run_args: &run_args::RunArgs) -> String {
    let mut pages_string = String::new();
    for (name, page) in pages {
        let templates = &page.templates;
        let paragraphs = gen_paragraphs(templates);
        let links = gen_links(templates);
        let image = gen_image(templates, run_args);

        let page_string = format!(
            r#"
        <div id="page-{name}" class="page">
        {image}
        {paragraphs}
        {links}
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
        run_args::RunArgs,
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
        let run_args = RunArgs {
            generation_dir: Default::default(),
            source_dir: Default::default(),
            max_log_level: Default::default(),
            watch: Default::default(),
        };
        let gen = gen_pages(&pages, &run_args);
        assert!(gen.contains("FirstPage"));
    }
}
