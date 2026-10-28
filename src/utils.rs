use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn is_directory<P: AsRef<Path>>(path: &P) -> bool {
    if let Ok(meta) = fs::metadata(path)
        && meta.is_dir()
    {
        true
    } else {
        false
    }
}

fn touch(path: impl AsRef<Path>) -> io::Result<()> {
    OpenOptions::new().write(true).create(true).open(path)?;

    Ok(())
}

pub fn touch_files_relative(base: PathBuf, files: Vec<PathBuf>, callback: impl Fn(&PathBuf)) {
    for f in files {
        let path = base.join(f);
        if path.has_root()
            && let Some(parent) = &path.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent).ok();
        }
        touch(&path).expect("failed to touch file");
        callback(&path);
    }
}
