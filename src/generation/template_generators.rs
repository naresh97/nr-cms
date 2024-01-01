use crate::{
    args, assets,
    types::{
        cms_blog::BlogPost,
        link_type::LinkType,
        template_type::{TemplateType, TemplateTypeVector},
    },
};

pub fn gen_title(templates: &Vec<TemplateType>) -> &str {
    match templates.get_title() {
        Some(title) => title,
        _ => "",
    }
}

pub fn gen_navbar(templates: &Vec<TemplateType>) -> String {
    match templates.get_navbar() {
        Some(paths) => {
            let paths: Vec<String> = paths
                .iter()
                .map(|x| format!("<a href=\"?page={x}\">{x}</a>"))
                .collect();
            let paths = paths.join(" | ");
            format!("<p>{paths}</p>")
        }
        _ => String::new(),
    }
}

pub fn gen_paragraphs(templates: &Vec<TemplateType>) -> String {
    let paragraphs = templates
        .get_paragraphs()
        .iter()
        .map(|x| format!("<p>{x}</p>"))
        .collect::<Vec<_>>();
    paragraphs.join("\n")
}

pub fn gen_links(templates: &Vec<TemplateType>) -> String {
    match templates.get_links() {
        Some(links) => {
            let links = links
                .iter()
                .map(|x| match x.0 {
                    LinkType::Github => format!(
                        r#"<a href="https://github.com/{}/">{} Github</a>"#,
                        x.1,
                        assets::svg_images::INVERTOCAT_SVG
                    ),
                })
                .collect::<Vec<_>>();
            let links = links.join(" | ");
            format!("<p>{links}</p>")
        }
        _ => String::new(),
    }
}

pub fn gen_nr_cms_info(templates: &Vec<TemplateType>) -> String {
    match templates.get_nr_cms_info() {
        Some(info) => {
            let info = format!("<p>{}</p>", info);
            info
        }
        _ => String::new(),
    }
}

pub fn gen_image(templates: &Vec<TemplateType>, generation_dirs: &args::GenerationDirs) -> String {
    match templates.get_image() {
        Some(image) => {
            if *image.1 {
                let result = match image.2 {
                    Some(x) => generation_dirs.copy_asset_img(image.0, *x),
                    _ => generation_dirs.copy_asset(image.0),
                };
                if result.is_err() {
                    return String::new();
                }
            }
            format!(r#"<p><img src="{}"/></p>"#, image.0)
        }
        _ => String::new(),
    }
}

pub fn gen_blog_post(post: &BlogPost) -> Option<String> {
    let templates = &post.templates;
    let title = templates.get_title()?;
    let date = templates.get_date()?;
    let date = date.timestamp_millis();
    let paragraphs = &templates.get_paragraphs();
    let paragraphs = paragraphs
        .iter()
        .map(|x| format!("<p>{x}</p>"))
        .collect::<Vec<_>>()
        .join("\n");
    Some(format!(
        r#"
    <div class="blog-post">
    <h2>{title}</h2>
    <span class="blog-post-date">{date}</span>
    {paragraphs}
    </div>
    "#
    ))
}

pub fn gen_blog(templates: &Vec<TemplateType>) -> String {
    if let Some(blog) = templates.get_blog() {
        let mut posts = blog.posts.clone();
        posts.sort_by(|a, b| a.post_date.cmp(&b.post_date));
        posts.reverse();
        let posts = posts
            .iter()
            .filter_map(gen_blog_post)
            .collect::<Vec<_>>()
            .join("\n<hr>\n");
        return format!(
            r#"
            <div class="blog">
            {posts}
            </div>
            "#
        );
    }
    String::new()
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use crate::types::cms_blog::CMSBlog;

    use super::*;

    #[test]
    fn test_blog_post() {
        let post = BlogPost {
            post_date: Default::default(),
            templates: Vec::from([
                TemplateType::Title {
                    title: "testtitle".to_string(),
                },
                TemplateType::Date {
                    date: chrono::Utc::now(),
                },
                TemplateType::Paragraph {
                    content: "testtest".to_string(),
                },
            ]),
        };
        let gen = gen_blog_post(&post).unwrap();
        assert!(gen.contains("testtest"));
        assert!(gen.contains("testtitle"));
    }

    #[test]
    fn test_blog() {
        let post = BlogPost {
            post_date: Default::default(),
            templates: Vec::from([
                TemplateType::Title {
                    title: "testtitle".to_string(),
                },
                TemplateType::Date {
                    date: chrono::Utc::now(),
                },
                TemplateType::Paragraph {
                    content: "testtest".to_string(),
                },
            ]),
        };
        let blog = CMSBlog {
            posts: Vec::from([post]),
        };
        let templates = Vec::from([TemplateType::Blog(blog)]);
        let gen = gen_blog(&templates);
        assert!(gen.contains("testtest"));
        assert!(gen.contains("testtitle"));
    }
    #[test]
    fn test_gen_title() {
        let test = vec![TemplateType::Title {
            title: "test".to_string(),
        }];
        assert_eq!(gen_title(&test), "test");
    }

    #[test]
    fn test_gen_navbar() {
        let test = vec![TemplateType::Navbar {
            paths: Vec::from(["first".to_string(), "second".to_string()]),
        }];
        let navbar = gen_navbar(&test);
        assert!(navbar.contains("first"));
        assert!(navbar.contains("second"));
    }

    #[test]
    fn test_gen_paragraph() {
        let test = vec![
            TemplateType::Paragraph {
                content: "first".to_string(),
            },
            TemplateType::Paragraph {
                content: "second".to_string(),
            },
        ];
        let paragraphs = gen_paragraphs(&test);
        assert!(paragraphs.contains("first"));
        assert!(paragraphs.contains("second"));
    }

    #[test]
    fn test_gen_links() {
        let test = vec![TemplateType::Links {
            links: HashMap::from([(LinkType::Github, "first".to_string())]),
        }];
        let links = gen_links(&test);
        assert!(links.contains("first"));
    }

    #[test]
    fn test_gen_info() {
        let test = vec![TemplateType::NRCMSInfo { text: "first" }];
        let info = gen_nr_cms_info(&test);
        assert!(info.contains("first"));
    }

    #[test]
    fn test_gen_image() {
        let generation_dirs = args::GenerationDirs {
            generation_dir: PathBuf::from("gen_gen_test/"),
            source_dir: PathBuf::from("sample/"),
        };

        let test = vec![TemplateType::Image {
            url: "sample.jpg".to_string(),
            copy_asset: true,
            size: Some(200),
        }];
        let image = gen_image(&test, &generation_dirs);
        assert!(image.contains("sample.jpg"));

        let test = vec![TemplateType::Image {
            url: "sample.jpg".to_string(),
            copy_asset: false,
            size: Some(200),
        }];
        let image = gen_image(&test, &generation_dirs);
        assert!(image.contains("sample.jpg"));

        std::fs::remove_dir_all("gen_gen_test/").unwrap();
    }

    #[test]
    fn test_image_no_exist() {
        let mut test = Vec::<TemplateType>::new();
        let generation_dirs = args::GenerationDirs {
            generation_dir: Default::default(),
            source_dir: Default::default(),
        };
        test.push(TemplateType::Image {
            url: "sample_no_exist.jpg".to_string(),
            copy_asset: true,
            size: Some(200),
        });
        let image = gen_image(&test, &generation_dirs);
        assert_eq!(image, String::new());

        let test = vec![TemplateType::Image {
            url: "sample_no_exist.jpg".to_string(),
            copy_asset: true,
            size: None,
        }];
        let image = gen_image(&test, &generation_dirs);
        assert_eq!(image, String::new());
    }
    #[test]
    fn test_no_templates() {
        let test = Vec::<TemplateType>::new();
        let generation_dirs = args::GenerationDirs {
            generation_dir: Default::default(),
            source_dir: Default::default(),
        };
        assert_eq!(gen_title(&test), String::new());
        assert_eq!(gen_paragraphs(&test), String::new());
        assert_eq!(gen_nr_cms_info(&test), String::new());
        assert_eq!(gen_links(&test), String::new());
        assert_eq!(gen_navbar(&test), String::new());
        assert_eq!(gen_image(&test, &generation_dirs), String::new());
    }
}
