use std::sync::mpsc::Receiver;

use notify::{RecursiveMode, Watcher};

use crate::{
    generation::generate_website::generate_website, types::generation_dirs::GenerationDirs,
};

fn watch_event(event: notify::Event, generation_dirs: impl GenerationDirs) {
    let handled = match event.kind {
        notify::EventKind::Create(_) => (true, "create"),
        notify::EventKind::Modify(_) => (true, "modify"),
        notify::EventKind::Remove(_) => (true, "remove"),
        _ => (false, "other"),
    };
    log::trace!(
        "Filesystem event path: {}",
        event
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
    generate_website(&generation_dirs);
}

fn watch_error(e: notify::Error) {
    log::error!("Filesystem Watcher Error: {}", e.to_string());
}

pub fn watch(
    generation_dirs: impl GenerationDirs,
    cancellation_token: Option<Receiver<bool>>,
) -> Result<(), notify::Error> {
    let source_dir = &generation_dirs.get_source_dir();
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => {
            watch_event(event, generation_dirs.clone());
        }
        Err(e) => watch_error(e),
    })?;
    watcher.watch(source_dir, RecursiveMode::Recursive)?;
    loop {
        let cancellation_token = cancellation_token.as_ref().and_then(|x| x.recv().ok());
        if let Some(cancellation_token) = cancellation_token {
            if cancellation_token {
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::types::generation_dirs::TempGenerationDirs;

    use super::*;
    #[test]
    fn test_watch_methods() {
        // Should not panic
        watch_event(
            notify::Event {
                kind: Default::default(),
                paths: Default::default(),
                attrs: Default::default(),
            },
            TempGenerationDirs::default(),
        );
        watch_event(
            notify::Event {
                kind: notify::EventKind::Modify(notify::event::ModifyKind::Data(
                    notify::event::DataChange::Any,
                )),
                paths: Default::default(),
                attrs: Default::default(),
            },
            TempGenerationDirs::default(),
        );

        // Should not panic
        watch_error(notify::Error {
            kind: notify::ErrorKind::Generic(Default::default()),
            paths: Default::default(),
        });
    }

    #[test]
    fn test_watch() {
        let (tx, rx) = std::sync::mpsc::channel::<bool>();
        let handle = std::thread::spawn(move || {
            watch(TempGenerationDirs::default(), Some(rx)).unwrap();
        });
        tx.send(false).unwrap();
        std::thread::sleep(Duration::from_millis(100));
        assert!(!handle.is_finished());
        tx.send(true).unwrap();
        for _ in 0..10 {
            if !handle.is_finished() {
                std::thread::sleep(Duration::from_millis(100));
            } else {
                handle.join().unwrap();
                return;
            }
        }
        panic!("Could not kill watcher thread");
    }
}
