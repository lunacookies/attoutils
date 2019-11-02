use anyhow::Result;
use clap::{App, Arg};
use std::env;

fn main() -> Result<()> {
    let matches = App::new("ls")
        .version("0.1")
        .author("Aramis Razzaghipour <aramisnoah@gmail.com>")
        .arg(
            Arg::with_name("targets")
                .help("Specifies which directories to list the contents of. Defaults to the current directory.")
                .multiple(true)
                .value_name("DIRECTORY")
        )
        .get_matches();

    let dirs: Vec<String> = matches
        .values_of_lossy("targets")
        .unwrap_or(vec![env::current_dir()?.to_string_lossy().into()]);

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
