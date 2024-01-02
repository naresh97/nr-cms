use std::path::{Path, PathBuf};

use crate::{
    parsing::parse_templates,
    types::{
        cms_blog::{BlogPost, CMSBlog},
        generation_dirs::GenerationDirs,
        template_type::{TemplateType, TemplateTypeVector},
    },
};

fn get_files_in_dir(path: &Path) -> Option<Vec<PathBuf>> {
    Some(
        std::fs::read_dir(path)
            .ok()?
            .filter_map(|x| {
                let x = x.ok()?;
                let file_type = x.file_type().ok()?;
                if file_type.is_file() {
                    return Some(x.path());
                }
                None
            })
            .collect::<Vec<_>>(),
    )
}

fn parse_blog_file(path: &Path, generation_dirs: &impl GenerationDirs) -> Option<BlogPost> {
    let contents = std::fs::read_to_string(path).ok()?;
    let (templates, _pages) = parse_templates(&contents, generation_dirs);
    let post_date = templates.get_date()?;

    Some(BlogPost {
        post_date: *post_date,
        templates,
    })
}

pub fn parse_blog(
    content: Option<&str>,
    generation_dirs: &impl GenerationDirs,
) -> Option<TemplateType> {
    let blog_dir = generation_dirs.in_source(content?);
    if !blog_dir.exists() {
        return None;
    }

    let blog_files = get_files_in_dir(&blog_dir)?;
    let blog_posts = blog_files
        .iter()
        .filter_map(|x| parse_blog_file(x, generation_dirs))
        .collect::<Vec<_>>();
    Some(TemplateType::Blog(CMSBlog { posts: blog_posts }))
}

#[cfg(test)]
mod test {

    use crate::types::generation_dirs::TempGenerationDirs;

    use super::*;
    #[test]
    fn test_get_files_in_dir() {
        let files = get_files_in_dir(std::path::Path::new("sample/blog_files")).unwrap();
        assert!(!files.is_empty());
        assert!(files.contains(&std::path::PathBuf::from(
            "sample/blog_files/my_first_toy.cms".to_string()
        )));
    }

    #[test]
    fn test_blog_file() {
        let path = PathBuf::from("sample/blog_files/my_first_toy.cms");
        assert!(path.exists());
        let generation_dirs = TempGenerationDirs::default();
        let blog = parse_blog_file(&path, &generation_dirs).unwrap();
        assert!(blog.templates.get_title().unwrap().contains("toy"));
    }

    #[test]
    fn test_parse_blog() {
        let content = Some("blog_files/");
        let generation_dirs = TempGenerationDirs::default();
        let blog = parse_blog(content, &generation_dirs).unwrap();
        let blog = blog.get_blog().unwrap();
        assert!(!blog.posts.is_empty());
    }
}
