use anyhow::Result;
use libatto;
use rayon::prelude::*;

fn main() -> Result<()> {
    let stdin = libatto::get_stdin()?;
    let mut output = stdin.lines().collect::<Vec<_>>();
    output.par_sort_unstable();

    println!("{}", output.join("\n"));

    Ok(())
}
