use libatto;
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
        let entries: Vec<_> = self.to_string_iter().collect();
        write!(f, "{}", entries.join("\n"))
    }
}

impl Dirs {
    fn to_string_iter(&self) -> impl Iterator<Item = String> + '_ {
        self.entries
            .iter()
            .map(|entry| entry.file_name())
            .map(|entry| entry.to_string_lossy().to_string())
    }

    pub fn columns(self) -> ColumnDirs {
        ColumnDirs::new(self)
    }

    pub fn remove_hidden(&mut self) {
        self.entries.retain(|entry| {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            !file_name.starts_with(".")
        });
    }
}

pub struct ColumnDirs(Dirs);

impl ColumnDirs {
    fn new(d: Dirs) -> Self {
        ColumnDirs(d)
    }
}

impl fmt::Display for ColumnDirs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entries: Vec<_> = self.0.to_string_iter().collect();
        let width = libatto::get_term_width_with_fallback(80);

        write!(f, "{}", libatto::columnate(&entries, width, 8))
    }
}

pub fn get_dir_contents<T: AsRef<Path> + std::convert::Into<PathBuf>>(path: T) -> io::Result<Dirs> {
    let contents: io::Result<Vec<_>> = fs::read_dir(&path)?.collect();
    Ok(Dirs {
        entries: contents?,
        path: path.into(),
    })
}
