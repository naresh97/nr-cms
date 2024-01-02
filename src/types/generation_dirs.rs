use std::path::PathBuf;

use crate::img_handling;

use super::program_args::ProgramArgs;

pub trait GenerationDirs: Clone + Send + 'static {
    fn get_source_dir(&self) -> PathBuf;
    fn get_generation_dir(&self) -> PathBuf;
    fn in_source(&self, path: &str) -> std::path::PathBuf {
        let mut pathbuf = std::path::PathBuf::new();
        pathbuf.push(self.get_source_dir());
        pathbuf.push(path);
        pathbuf
    }
    fn in_gen(&self, path: &str) -> std::path::PathBuf {
        let mut pathbuf = std::path::PathBuf::new();
        pathbuf.push(self.get_generation_dir());
        pathbuf.push(path);
        pathbuf
    }
    fn copy_asset(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
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
        Ok(())
    }
    fn copy_asset_img(&self, path: &str, size: u32) -> Result<(), Box<dyn std::error::Error>> {
        let target = self.in_gen(path);
        let source = self.in_source(path);
        img_handling::resize_image(&source, &target, size)?;
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct StandardGenerationDirs {
    source_dir: std::path::PathBuf,
    generation_dir: std::path::PathBuf,
}

impl GenerationDirs for StandardGenerationDirs {
    fn get_source_dir(&self) -> PathBuf {
        self.source_dir.to_path_buf()
    }

    fn get_generation_dir(&self) -> PathBuf {
        self.generation_dir.to_path_buf()
    }
}

impl From<ProgramArgs> for StandardGenerationDirs {
    fn from(value: ProgramArgs) -> Self {
        StandardGenerationDirs {
            source_dir: PathBuf::from(value.source_dir),
            generation_dir: PathBuf::from(value.generation_dir),
        }
    }
}

pub struct TempGenerationDirs {
    tmp_dir: Option<tempfile::TempDir>,
}

impl Clone for TempGenerationDirs {
    fn clone(&self) -> Self {
        Self {
            tmp_dir: tempfile::tempdir().ok(),
        }
    }
}

impl GenerationDirs for TempGenerationDirs {
    fn get_source_dir(&self) -> PathBuf {
        PathBuf::from("sample/")
    }

    fn get_generation_dir(&self) -> std::path::PathBuf {
        self.tmp_dir
            .as_ref()
            .map(|x| x.path().to_path_buf())
            .unwrap_or(PathBuf::from("gen_tmp/"))
    }
}

impl Default for TempGenerationDirs {
    fn default() -> Self {
        Self {
            tmp_dir: tempfile::tempdir().ok(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_program_args_conversion() {
        let program_args = ProgramArgs {
            source_dir: "first/".to_string(),
            generation_dir: "second/".to_string(),
            max_log_level: Default::default(),
            watch: Default::default(),
        };
        let generation_dirs = StandardGenerationDirs::from(program_args);
        assert_eq!("first/", generation_dirs.source_dir.to_str().unwrap());
        assert_eq!("second/", generation_dirs.generation_dir.to_str().unwrap());
    }
    #[test]
    fn test_path_finding() {
        let generation_dirs = TempGenerationDirs::default();
        assert_eq!(
            generation_dirs.in_source("test"),
            PathBuf::from("sample/test")
        );
        assert_eq!(
            generation_dirs.in_gen("test"),
            generation_dirs.get_generation_dir().join("test")
        );
    }
    #[test]
    fn test_copy_img_resize() {
        let generation_dirs = TempGenerationDirs::default();
        generation_dirs.copy_asset_img("sample.jpg", 200).unwrap();
        assert!(generation_dirs.in_gen("sample.jpg").exists());
    }
    #[test]
    fn test_copy_asset() {
        let generation_dirs = TempGenerationDirs::default();
        generation_dirs.copy_asset("sample.jpg").unwrap();
        assert!(generation_dirs.in_gen("sample.jpg").exists());
    }
}
