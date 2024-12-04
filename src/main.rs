#![warn(clippy::pedantic)]
#![deny(clippy::unused_result_ok)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

mod expression_parser;
mod expressions;
mod markdown_translator;
mod site_builder;
mod site_generator;
mod utils;

use std::{path::Path, sync::mpsc};

use expression_parser::ExpressionParser;
use notify::{EventKind, Watcher};
use site_builder::SiteBuilder;
use site_generator::SiteGenerator;
use unicode_segmentation::UnicodeSegmentation;

#[allow(clippy::restriction)]
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let watch = args.get(1).is_some_and(|arg| arg == "watch");
    if watch {
        let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
        let mut watcher = notify::recommended_watcher(tx).unwrap();
        watcher
            .watch(Path::new("./site"), notify::RecursiveMode::Recursive)
            .unwrap();
        for res in rx.into_iter().flatten() {
            match res.kind {
                EventKind::Create(_)
                | EventKind::Modify(_)
                | EventKind::Remove(_)
                | EventKind::Other => (),
                _ => continue,
            };
            if let Err(e) = generate_site("./site/start.txt") {
                println!("{e}");
                return;
            }
            println!("Updated!");
        }
        return;
    }
    generate_site("./site/start.txt").unwrap();
}

fn generate_site(start_file: &str) -> anyhow::Result<()> {
    let text = std::fs::read_to_string(start_file)?;
    let text = text.graphemes(true).collect::<Vec<_>>();

    let expression_parser = ExpressionParser::new(text);
    let (expressions, errors) = expression_parser.parse();
    for e in errors {
        println!("Expression Error: {e}");
    }

    let site_builder = SiteBuilder::new(expressions);
    let site = site_builder.parse()?;

    let site_generator = SiteGenerator::new(site);
    let pages = site_generator.generate();

    if std::fs::exists("./output")? {
        std::fs::remove_dir_all("./output")?;
    }
    std::fs::create_dir("./output")?;

    for page in pages {
        std::fs::write(format!("./output/{}", page.filename), page.content)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {}
