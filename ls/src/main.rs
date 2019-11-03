use anyhow::Result;
use clap::{App, Arg};
use rayon::prelude::*;
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
        .arg(
            Arg::with_name("single_column")
                .help("Displays directory listings in a single column. Defaults to false.")
                .long("single-column")
                .short("1")
        )
        .get_matches();

    let dirs: Vec<String> = matches
        .values_of_lossy("targets")
        .unwrap_or(vec![env::current_dir()?.to_string_lossy().into()]);

    let mut listings = Vec::with_capacity(dirs.len());

    for dir in dirs.iter() {
        listings.push(ls::get_dir_contents(dir)?);
    }

    // We use rayon’s par_sort_unstable_by() so that comparisons are based on the entry’s file name.
    for listing in listings.iter_mut() {
        listing
            .entries
            .par_sort_unstable_by(|a, b| a.file_name().cmp(&b.file_name()));
    }

    listings.into_iter().enumerate().for_each(|(i, listing)| {
        if dirs.len() > 1 {
            // Add an empty line between directory listings.
            if i > 0 {
                println!("");
            }
            println!("{}:", listing.path.to_string_lossy());
        };

        if matches.is_present("single_column") {
            println!("{}", listing);
        } else {
            let listing = listing.columns();
            println!("{}", listing);
        }
    });

    Ok(())
}
