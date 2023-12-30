mod assets;
mod gen;
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
        generation_dir: String::from("./gen"),
        source_dir: String::from("./sample"),
        max_log_level: Some(String::from("debug")),
    };
    init_logging(&run_args);
    log::info!("Starting NKR-CMS.");
    log::debug!("Running with arguments");
    log::debug!("Source Directory: {}", &run_args.source_dir);
    log::debug!("Generation Directory: {}", &run_args.generation_dir);
    if let Some(max_log_level) = &run_args.max_log_level {
        log::debug!("Max Log Level: {}", max_log_level);
    }
    if let Err(e) = watcher::watch(run_args) {
        log::error!("Could not initialize watcher: {e}");
    }
}
