use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let mut dirs: Vec<_> = env::args().skip(1).collect();

    if dirs.is_empty() {
        dirs.push(env::current_dir()?.to_string_lossy().into())
    }

    let mut listings = Vec::with_capacity(dirs.len());

    for dir in dirs.iter() {
        listings.push(ls::get_dir_contents(dir)?);
    }

    listings.iter().for_each(|listing| {
        if dirs.len() > 1 {
            println!("{}:", listing.path.to_string_lossy());
        };

        println!("{}", listing);
    });

    Ok(())
}
