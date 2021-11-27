#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn hoge() {
    println!("randompicklib::hoge() called");
}

use std::fs;
use std::path::{Path, PathBuf};

pub fn get_files(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let dir = fs::read_dir(path)?;
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in dir.into_iter() {
        files.push(entry?.path());
    }
    Ok(files)
}
