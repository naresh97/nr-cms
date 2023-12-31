use crate::{
    assets, run_args,
    types::{
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
            return info;
        }
        _ => String::new(),
    }
}

pub fn gen_image(templates: &Vec<TemplateType>, run_args: &run_args::RunArgs) -> String {
    match templates.get_image() {
        Some(image) => {
            if *image.1 {
                let result = match image.2 {
                    Some(x) => run_args.copy_asset_img(image.0, *x),
                    _ => run_args.copy_asset(image.0),
                };
                if result.is_err() {
                    return String::new();
                }
            }
            return format!(r#"<p><img src="{}"/></p>"#, image.0);
        }
        _ => String::new(),
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn test_gen_title() {
        let mut test = Vec::<TemplateType>::new();
        test.push(TemplateType::Title {
            title: "test".to_string(),
        });
        assert_eq!(gen_title(&test), "test");
    }

    #[test]
    fn test_gen_navbar() {
        let mut test = Vec::<TemplateType>::new();
        test.push(TemplateType::Navbar {
            paths: Vec::from(["first".to_string(), "second".to_string()]),
        });
        let navbar = gen_navbar(&test);
        assert!(navbar.contains("first"));
        assert!(navbar.contains("second"));
    }

    #[test]
    fn test_gen_paragraph() {
        let mut test = Vec::<TemplateType>::new();
        test.push(TemplateType::Paragraph {
            content: "first".to_string(),
        });
        test.push(TemplateType::Paragraph {
            content: "second".to_string(),
        });
        let paragraphs = gen_paragraphs(&test);
        assert!(paragraphs.contains("first"));
        assert!(paragraphs.contains("second"));
    }

    #[test]
    fn test_gen_links() {
        let mut test = Vec::<TemplateType>::new();
        test.push(TemplateType::Links {
            links: HashMap::from([(LinkType::Github, "first".to_string())]),
        });
        let links = gen_links(&test);
        assert!(links.contains("first"));
    }

    #[test]
    fn test_gen_info() {
        let mut test = Vec::<TemplateType>::new();
        test.push(TemplateType::NRCMSInfo { text: "first" });
        let info = gen_nr_cms_info(&test);
        assert!(info.contains("first"));
    }

    #[test]
    fn test_gen_image() {
        let run_args = run_args::RunArgs {
            generation_dir: "gen_gen_test/".to_string(),
            source_dir: "sample/".to_string(),
            max_log_level: None,
        };

        let mut test = Vec::<TemplateType>::new();
        test.push(TemplateType::Image {
            url: "sample.jpg".to_string(),
            copy_asset: true,
            size: Some(200),
        });
        let image = gen_image(&test, &run_args);
        assert!(image.contains("sample.jpg"));

        let mut test = Vec::<TemplateType>::new();
        test.push(TemplateType::Image {
            url: "sample.jpg".to_string(),
            copy_asset: false,
            size: Some(200),
        });
        let image = gen_image(&test, &run_args);
        assert!(image.contains("sample.jpg"));

        std::fs::remove_dir_all("gen_gen_test/").unwrap();
    }

    #[test]
    fn test_image_no_exist() {
        let mut test = Vec::<TemplateType>::new();
        let run_args = run_args::RunArgs {
            generation_dir: Default::default(),
            source_dir: Default::default(),
            max_log_level: None,
        };
        test.push(TemplateType::Image {
            url: "sample_no_exist.jpg".to_string(),
            copy_asset: true,
            size: Some(200),
        });
        let image = gen_image(&test, &run_args);
        assert_eq!(image, String::new());

        let mut test = Vec::<TemplateType>::new();
        test.push(TemplateType::Image {
            url: "sample_no_exist.jpg".to_string(),
            copy_asset: true,
            size: None,
        });
        let image = gen_image(&test, &run_args);
        assert_eq!(image, String::new());
    }
    #[test]
    fn test_no_templates() {
        let test = Vec::<TemplateType>::new();
        let run_args = run_args::RunArgs {
            generation_dir: Default::default(),
            source_dir: Default::default(),
            max_log_level: Default::default(),
        };
        assert_eq!(gen_title(&test), String::new());
        assert_eq!(gen_paragraphs(&test), String::new());
        assert_eq!(gen_nr_cms_info(&test), String::new());
        assert_eq!(gen_links(&test), String::new());
        assert_eq!(gen_navbar(&test), String::new());
        assert_eq!(gen_image(&test, &run_args), String::new());
    }
}
