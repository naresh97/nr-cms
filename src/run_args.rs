use clap::Parser;

use crate::img_handling;

#[derive(Clone, Parser)]
#[command(author,version,about,long_about=None)]
pub struct RunArgs {
    pub source_dir: String,
    #[arg(default_value_t = String::from("gen/"))]
    pub generation_dir: String,
    #[arg(short, long, default_value_t = String::from("info"))]
    pub max_log_level: String,
    #[arg(short, long)]
    pub watch: bool,
}

impl RunArgs {
    pub fn in_source(&self, path: &str) -> std::path::PathBuf {
        let mut pathbuf = std::path::PathBuf::new();
        pathbuf.push(&self.source_dir);
        pathbuf.push(path);
        pathbuf
    }
    pub fn in_gen(&self, path: &str) -> std::path::PathBuf {
        let mut pathbuf = std::path::PathBuf::new();
        pathbuf.push(&self.generation_dir);
        pathbuf.push(path);
        pathbuf
    }
    pub fn copy_asset(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let target = self.in_gen(path);
        let source = self.in_source(path);

        let target_parent = target.parent().ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "Could not get parent dir of {}",
                target.to_str().unwrap_or("?")
            ),
        ))?;
        std::fs::create_dir_all(target_parent)?;
        std::fs::copy(source, target)?;
        return Ok(());
    }
    pub fn copy_asset_img(&self, path: &str, size: u32) -> Result<(), Box<dyn std::error::Error>> {
        let target = self.in_gen(path);
        let source = self.in_source(path);
        img_handling::resize_image(&source, &target, size)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_path_finding() {
        let run_args = RunArgs {
            generation_dir: "gen/".to_string(),
            source_dir: "sample/".to_string(),
            max_log_level: Default::default(),
            watch: Default::default(),
        };
        assert_eq!(run_args.in_source("test"), PathBuf::from("sample/test"));
        assert_eq!(run_args.in_gen("test"), PathBuf::from("gen/test"));
    }
    #[test]
    fn test_copy_img_resize() {
        let run_args = RunArgs {
            generation_dir: "gen_testa/".to_string(),
            source_dir: "sample/".to_string(),
            max_log_level: Default::default(),
            watch: Default::default(),
        };
        run_args.copy_asset_img("sample.jpg", 200).unwrap();
        assert!(PathBuf::from("gen_testa/sample.jpg").exists());
        std::fs::remove_dir_all("gen_testa/").unwrap();
    }
    #[test]
    fn test_copy_asset() {
        let run_args = RunArgs {
            generation_dir: "gen_testb/".to_string(),
            source_dir: "sample/".to_string(),
            max_log_level: Default::default(),
            watch: Default::default(),
        };
        run_args.copy_asset("sample.jpg").unwrap();
        assert!(PathBuf::from("gen_testb/sample.jpg").exists());
        std::fs::remove_dir_all("gen_testb/").unwrap();
    }
    #[test]
    fn test_clone() {
        let run_args = RunArgs {
            generation_dir: "gen/".to_string(),
            source_dir: "sample/".to_string(),
            max_log_level: Default::default(),
            watch: Default::default(),
        };
        let clone = run_args.clone();
        assert_eq!(run_args.generation_dir, clone.generation_dir);
    }
}
