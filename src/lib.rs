use std::fs;
use std::path::{Path, PathBuf};

pub fn get_files(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let dir = fs::read_dir(path)?;
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in dir.into_iter() {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            let mut dir_files = get_files(&entry.path())?;
            files.append(&mut dir_files);
        } else if file_type.is_file() {
            files.push(entry.path());
        } else {
            panic!(
                "This file is neither a directory nor a file. {:?}",
                entry.path()
            )
        }
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn get_files_works() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/dir1");
        let files = get_files(&path).unwrap();
        let mut files: Vec<&OsStr> = files
            .iter()
            .map(|path| path.as_path().file_name())
            .flatten()
            .collect();
        files.sort();
        assert_eq!(files.len(), 3);
        assert_eq!(files[0], OsStr::new("file1_in_dir1"));
        assert_eq!(files[1], OsStr::new("file2_in_dir2"));
        assert_eq!(files[2], OsStr::new("file3_in_dir3"));
    }
}
