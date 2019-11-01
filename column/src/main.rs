use anyhow::Result;
use libatto;

fn main() -> Result<()> {
    let stdin = libatto::get_stdin()?;

    // Default to width of 80 chars if it can’t be determined.
    let width = match column::get_term_width() {
        Ok(width) => width,
        _ => 80,
    };

    // TODO: find a way to remove this collect() to avoid allocation.
    println!(
        "{}",
        column::columnate(&stdin.lines().collect::<Vec<_>>(), width, 8)?
    );

    Ok(())
}
