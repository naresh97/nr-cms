use std::sync::mpsc::Receiver;

use notify::{RecursiveMode, Watcher};

use crate::{generation::generate_website::generate_website, run_args};

fn watch_event(event: notify::Event, run_args: run_args::RunArgs) {
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
    generate_website(&run_args);
}

fn watch_error(e: notify::Error) {
    log::error!("Filesystem Watcher Error: {}", e.to_string());
}

pub fn watch(
    run_args: run_args::RunArgs,
    cancellation_token: Option<Receiver<bool>>,
) -> Result<(), notify::Error> {
    let source_dir = &run_args.source_dir.clone();
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => watch_event(event, run_args.clone()),
        Err(e) => watch_error(e),
    })?;
    watcher.watch(std::path::Path::new(source_dir), RecursiveMode::Recursive)?;
    loop {
        let cancellation_token = cancellation_token.as_ref().map(|x| x.recv().ok()).flatten();
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

    use crate::run_args::RunArgs;

    use super::*;
    #[test]
    fn test_watch_methods() {
        let run_args = RunArgs {
            generation_dir: Default::default(),
            source_dir: Default::default(),
            max_log_level: Default::default(),
            watch: Default::default(),
        };

        // Should not panic
        watch_event(
            notify::Event {
                kind: Default::default(),
                paths: Default::default(),
                attrs: Default::default(),
            },
            run_args.clone(),
        );
        watch_event(
            notify::Event {
                kind: notify::EventKind::Modify(notify::event::ModifyKind::Data(
                    notify::event::DataChange::Any,
                )),
                paths: Default::default(),
                attrs: Default::default(),
            },
            run_args.clone(),
        );

        // Should not panic
        watch_error(notify::Error {
            kind: notify::ErrorKind::Generic(Default::default()),
            paths: Default::default(),
        });
    }

    #[test]
    fn test_watch() {
        let run_args = RunArgs {
            generation_dir: Default::default(),
            source_dir: Default::default(),
            max_log_level: Default::default(),
            watch: Default::default(),
        };
        let (tx, rx) = std::sync::mpsc::channel::<bool>();
        let handle = std::thread::spawn(move || {
            watch(run_args, Some(rx)).unwrap();
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
