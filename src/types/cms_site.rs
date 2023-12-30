use std::collections::HashMap;

use super::{cms_page::CMSPage, template_type::TemplateType};

pub struct CMSSite {
    pub original_content: String,
    pub templates: Vec<TemplateType>,
    pub pages: HashMap<String, CMSPage>,
}
