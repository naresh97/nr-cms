use notify::{RecursiveMode, Watcher};

use crate::cms_types::CMSSite;
use crate::gen::generate_website;
use crate::{parsing, run_args};

fn load_cms_site(
    file_path: std::path::PathBuf,
    run_args: &run_args::RunArgs,
) -> Result<CMSSite, std::io::Error> {
    println!("Loading CMS files");
    let contents = std::fs::read_to_string(file_path)?;
    let (templates, pages) = parsing::parse_templates(&contents, run_args);
    let cms_site = CMSSite {
        original_content: contents,
        templates,
        pages,
    };

    return Ok(cms_site);
}

fn write_gen_site(
    file_path: std::path::PathBuf,
    cms_site: &CMSSite,
    run_args: &run_args::RunArgs,
) -> Result<(), std::io::Error> {
    log::info!("Generating website");
    let gen_file = generate_website(&cms_site, run_args);
    let parent = file_path.parent().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not get parent",
    ))?;
    std::fs::create_dir_all(parent)?;
    std::fs::write(file_path, gen_file)?;
    return Ok(());
}

fn watch_event(_event: notify::Event, run_args: run_args::RunArgs) {
    let handled = match _event.kind {
        notify::EventKind::Any => (false, "any"),
        notify::EventKind::Access(_) => (false, "access"),
        notify::EventKind::Create(_) => (true, "create"),
        notify::EventKind::Modify(_) => (true, "modify"),
        notify::EventKind::Remove(_) => (true, "remove"),
        notify::EventKind::Other => (false, "other"),
    };
    log::trace!(
        "Filesystem event path: {}",
        _event
            .paths
            .iter()
            .filter_map(|x| x.to_str())
            .collect::<Vec<_>>()
            .join(",")
    );
    log::trace!("Filesystem event kind: {}", handled.1);
    if !handled.0 {
        return;
    }
    log::info!("Filesystem change detected");
    let index_file = load_cms_site(run_args.in_source("index.cms"), &run_args);
    match index_file {
        Ok(index_file) => {
            let result = write_gen_site(run_args.in_gen("index.html"), &index_file, &run_args);
            if let Err(e) = result {
                log::error!("Could not generate site: {}", e.to_string());
            }
        }
        Err(e) => {
            log::error!("Could not load CMS site: {}", e.to_string());
        }
    };
}

fn watch_error(e: notify::Error) {
    log::error!("Filesystem Watcher Error: {}", e.to_string());
}

pub fn watch(run_args: run_args::RunArgs) -> Result<(), notify::Error> {
    let source_dir = &run_args.source_dir.clone();
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => watch_event(event, run_args.clone()),
        Err(e) => watch_error(e),
    })?;
    watcher.watch(std::path::Path::new(source_dir), RecursiveMode::Recursive)?;
    loop {}
}
