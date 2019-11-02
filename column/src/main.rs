use anyhow::Result;
use libatto;

fn main() -> Result<()> {
    let stdin = libatto::get_stdin()?;
    let width = libatto::get_term_width_with_fallback(80);

    // TODO: find a way to remove this collect() to avoid allocation.
    println!(
        "{}",
        libatto::columnate(&stdin.lines().collect::<Vec<_>>(), width, 8)
    );

    Ok(())
}
