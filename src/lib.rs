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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn get_files_works() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/dir1");
        let files = get_files(&path).unwrap();
        let files: Vec<Option<&OsStr>> = files
            .iter()
            .map(|path| path.as_path().file_name())
            .collect();
        assert_eq!(files.len(), 2);
        // TODO: readdirで返るファイルの順序は保証されていないのでsortした方がいい
        assert_eq!(files[0], Some(OsStr::new("dir2")));
        assert_eq!(files[1], Some(OsStr::new("file1_in_dir1")));
    }
}
