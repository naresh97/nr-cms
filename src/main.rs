mod assets;
mod cms_types;
mod gen;
mod img_handling;
mod parsing;
mod run_args;
mod watcher;

fn main() {
    let run_args = run_args::RunArgs {
        generation_dir: String::from("./gen"),
        source_dir: String::from("./sample"),
    };
    watcher::watch(run_args).expect("Could not initialize filesystem watcher");
}
