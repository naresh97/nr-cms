use clap::Parser;

use crate::generation::generate_website::generate_website;

mod args;
mod assets;
mod generation;
mod img_handling;
mod parsing;
mod types;
mod watcher;

fn init_logging(run_args: &args::ProgramArgs) {
    let mut env = env_logger::Env::default();
    env = env.filter_or("RUST_LOG", &run_args.max_log_level);
    env_logger::init_from_env(env);
}

fn main() {
    let run_args = args::ProgramArgs::parse();
    //let run_args = parse_args();
    init_logging(&run_args);
    log::info!("Starting NKR-CMS.");
    log::debug!("Running with arguments");
    log::debug!("Source Directory: {}", &run_args.source_dir);
    log::debug!("Generation Directory: {}", &run_args.generation_dir);
    log::debug!("Max Log Level: {}", &run_args.max_log_level);
    if run_args.watch {
        log::info!("Running in watch mode.");
        if let Err(e) = watcher::watch(run_args.into(), None) {
            log::error!("Could not initialize watcher: {e}");
        }
    } else {
        generate_website(&run_args.into());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_init_logging() {
        let run_args = args::ProgramArgs {
            max_log_level: "off".to_string(),
            ..Default::default()
        };
        init_logging(&run_args);
        assert_eq!(log::max_level(), log::LevelFilter::Off);
    }
}
