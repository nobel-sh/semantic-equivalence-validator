use log::info;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub fn get_files_in_dir(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    info!("Reading files from directory: {}", dir.display());
    let files = read_dir(dir)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();
    Ok(files)
}
