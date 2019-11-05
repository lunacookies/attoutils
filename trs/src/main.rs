use anyhow::Result;
use rayon::prelude::*;
use std::env;

fn main() -> Result<()> {
    let trash_dir = trs::get_trash_dir()?;
    trs::ensure_dir_exists(&trash_dir)?;

    let args: Vec<_> = env::args().collect();

    args.par_iter()
        .skip(1)
        .map(|arg| {
            let mut path = env::current_dir()?;
            path.push(arg);
            trs::move_to_trash(&path, &trash_dir)
        })
        .collect::<Result<_>>()?;

    Ok(())
}
