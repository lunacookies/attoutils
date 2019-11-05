use anyhow::{anyhow, Context, Result};
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

pub fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    Ok(())
}

pub fn get_trash_dir() -> Result<PathBuf> {
    use directories::ProjectDirs;

    match ProjectDirs::from("com", "arzg", "trs") {
        Some(proj_dirs) => Ok(proj_dirs.data_dir().into()),
        None => Err(anyhow!("failed locating trash directory")),
    }
}

fn get_file_name<'a>(path: &'a Path) -> Result<&'a OsStr> {
    path.file_name()
        .ok_or(anyhow!("path ended in .."))
        .with_context(|| format!("could not disambiguate path {}", path.display()))
}

fn disambiguate_path(path: &Path) -> Result<PathBuf> {
    let mut path = path.to_owned();
    let mut file_name = get_file_name(&path)?.to_owned();

    // Account for the possible existence of the path we’re handling plus ‘ Copy’ by continually
    // appending this until the path does not exist.
    while path.exists() {
        file_name.push(" Copy");
        path.set_file_name(&file_name);
    }

    Ok(path)
}

pub fn move_to_trash(path: &Path, trash_dir: &Path) -> Result<()> {
    let mut target_path = trash_dir.to_owned();
    target_path.push(get_file_name(path)?);
    target_path = disambiguate_path(&target_path)?;

    fs::rename(path, &target_path).with_context(|| {
        format!(
            "tried to trash path ‘{}’ to new path ‘{}’",
            path.display(),
            target_path.display()
        )
    })?;

    Ok(())
}
