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
    pub fn copy_asset(&self, path: &str) -> Result<(), std::io::Error> {
        use std::path::PathBuf;
        let mut target = PathBuf::new();
        target.push(&self.generation_dir);
        target.push(path);
        let target = target;

        let mut source = PathBuf::new();
        source.push(&self.source_dir);
        source.push(path);
        let source = source;

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
}
