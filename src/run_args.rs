use crate::img_handling;

#[derive(Clone)]
pub struct RunArgs {
    pub generation_dir: String,
    pub source_dir: String,
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
