use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::path::{Path, PathBuf};

pub fn random_pick(path: &Path) -> Option<PathBuf> {
    let files = get_files(path).unwrap();
    pick_one(files)
}

fn get_files(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let dir = fs::read_dir(path)?;
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in dir {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            let mut dir_files = get_files(&entry.path())?;
            files.append(&mut dir_files);
        } else if file_type.is_file() {
            files.push(entry.path());
        } else if file_type.is_symlink() {
            // ignore
        } else {
            panic!(
                "This file is neither a directory nor a file. {:?}",
                entry.path()
            )
        }
    }
    Ok(files)
}

fn pick_one(files: Vec<PathBuf>) -> Option<PathBuf> {
    let mut rng = thread_rng();
    return files.choose(&mut rng).cloned();
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
            .filter_map(|path| path.as_path().file_name())
            .collect();
        files.sort();
        assert_eq!(files.len(), 3);
        assert_eq!(files[0], OsStr::new("file1_in_dir1"));
        assert_eq!(files[1], OsStr::new("file2_in_dir2"));
        assert_eq!(files[2], OsStr::new("file3_in_dir3"));
    }

    #[test]
    fn get_katakana_dakuten_file_name() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/katakana_dakuten");
        let file = random_pick(&path).unwrap();
        let file_name = file.file_name().unwrap();
        let file_name_str = file_name.to_str().unwrap();
        assert_eq!(file_name_str, "ツハ\u{3099}キ");
        assert_ne!(file_name_str, "ツバキ");
    }

    #[test]
    fn ignore_symlink() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/symlink");
        let files = get_files(&path).unwrap();
        let files: Vec<&OsStr> = files
            .iter()
            .filter_map(|path| path.as_path().file_name())
            .collect();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], OsStr::new("file1"));
    }
}
