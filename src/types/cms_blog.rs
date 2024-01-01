use chrono::{self, Utc};

use super::template_type::TemplateType;

#[derive(Clone)]
pub struct BlogPost {
    pub post_date: chrono::DateTime<Utc>,
    pub templates: Vec<TemplateType>,
}

#[derive(Clone)]
pub struct CMSBlog {
    pub posts: Vec<BlogPost>,
}
