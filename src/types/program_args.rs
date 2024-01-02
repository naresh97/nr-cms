use clap::Parser;

#[derive(Parser, Default)]
#[command(author,version,about,long_about=None)]
pub struct ProgramArgs {
    pub source_dir: String,
    #[arg(default_value_t = String::from("gen/"))]
    pub generation_dir: String,
    #[arg(short, long, default_value_t = String::from("info"))]
    pub max_log_level: String,
    #[arg(short, long)]
    pub watch: bool,
}
