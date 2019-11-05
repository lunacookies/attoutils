use anyhow::Result;
use rayon::prelude::*;
use std::{
    convert::TryFrom,
    env,
    fs::File,
    io::{self, Read},
};

fn main() -> Result<()> {
    // Collect into a Vec here to leverage collect()’s ability to extract a single Result from an
    // Iterator over them.
    let mut files: Vec<_> = env::args()
        .skip(1)
        .map(|file| File::open(file))
        .collect::<io::Result<_>>()?;

    let total_size = files
        .par_iter()
        .map(|file: &File| file.metadata())
        .collect::<io::Result<Vec<_>>>()?
        .iter()
        .map(|metadata| metadata.len())
        .sum::<u64>();

    let mut output = String::with_capacity(usize::try_from(total_size)?);

    for file in files.iter_mut() {
        file.read_to_string(&mut output)?;
    }

    println!("{}", output);

    Ok(())
}
