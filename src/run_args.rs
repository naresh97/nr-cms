use crate::img_handling;

#[derive(Clone)]
pub struct RunArgs {
    pub generation_dir: String,
    pub source_dir: String,
    pub max_log_level: Option<String>,
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
            max_log_level: None,
        };
        assert_eq!(run_args.in_source("test"), PathBuf::from("sample/test"));
        assert_eq!(run_args.in_gen("test"), PathBuf::from("gen/test"));
    }
    #[test]
    fn test_copy_img_resize() {
        let run_args = RunArgs {
            generation_dir: "gen_testa/".to_string(),
            source_dir: "sample/".to_string(),
            max_log_level: None,
        };
        run_args.copy_asset_img("sample.jpg", 200).expect("");
        assert!(PathBuf::from("gen_testa/sample.jpg").exists());
        std::fs::remove_dir_all("gen_testa/").expect("");
    }
    #[test]
    fn test_copy_asset() {
        let run_args = RunArgs {
            generation_dir: "gen_testb/".to_string(),
            source_dir: "sample/".to_string(),
            max_log_level: None,
        };
        run_args.copy_asset("sample.jpg").expect("");
        assert!(PathBuf::from("gen_testb/sample.jpg").exists());
        std::fs::remove_dir_all("gen_testb/").expect("");
    }
    #[test]
    fn test_clone() {
        let run_args = RunArgs {
            generation_dir: "gen/".to_string(),
            source_dir: "sample/".to_string(),
            max_log_level: None,
        };
        let clone = run_args.clone();
        assert_eq!(run_args.generation_dir, clone.generation_dir);
    }
}
