mod assets;
mod generation;
mod img_handling;
mod parsing;
mod run_args;
mod types;
mod watcher;

fn init_logging(run_args: &run_args::RunArgs) {
    let mut env = env_logger::Env::default();
    if let Some(max_log_level) = &run_args.max_log_level {
        env = env.filter_or("RUST_LOG", max_log_level);
    }
    env_logger::init_from_env(env);
}

fn main() {
    let run_args = run_args::RunArgs {
        generation_dir: "./gen".to_string(),
        source_dir: "./sample".to_string(),
        max_log_level: Some("debug".to_string()),
    };
    init_logging(&run_args);
    log::info!("Starting NKR-CMS.");
    log::debug!("Running with arguments");
    log::debug!("Source Directory: {}", &run_args.source_dir);
    log::debug!("Generation Directory: {}", &run_args.generation_dir);
    if let Some(max_log_level) = &run_args.max_log_level {
        log::debug!("Max Log Level: {}", max_log_level);
    }
    if let Err(e) = watcher::watch(run_args, None) {
        log::error!("Could not initialize watcher: {e}");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_init_logging() {
        let run_args = run_args::RunArgs {
            generation_dir: Default::default(),
            source_dir: Default::default(),
            max_log_level: Some("off".to_string()),
        };
        init_logging(&run_args);
        assert_eq!(log::max_level(), log::LevelFilter::Off);
    }
}
