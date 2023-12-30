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
