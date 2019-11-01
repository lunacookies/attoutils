use std::{
    fmt,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

pub struct Dirs {
    pub entries: Vec<DirEntry>,
    pub path: PathBuf,
}

impl fmt::Display for Dirs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output: Vec<_> = self
            .entries
            .iter()
            .map(|entry| entry.file_name())
            .map(|entry| entry.to_string_lossy().to_string())
            .collect();

        write!(f, "{}", output.join("\n"))
    }
}

pub fn get_dir_contents<T: AsRef<Path> + std::convert::Into<PathBuf>>(path: T) -> io::Result<Dirs> {
    let contents: io::Result<Vec<_>> = fs::read_dir(&path)?.collect();
    Ok(Dirs {
        entries: contents?,
        path: path.into(),
    })
}
