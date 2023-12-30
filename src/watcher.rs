use notify::{RecursiveMode, Watcher};

use crate::{generation::generate_website::load_and_write_site, run_args};

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
    load_and_write_site(&run_args);
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
