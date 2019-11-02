use anyhow::Result;
use libatto;

fn main() -> Result<()> {
    let stdin = libatto::get_stdin()?;

    // Default to width of 80 chars if it can’t be determined.
    let width = match get_term_width() {
        Ok(width) => width,
        _ => 80,
    };

    // TODO: find a way to remove this collect() to avoid allocation.
    println!(
        "{}",
        libatto::columnate(&stdin.lines().collect::<Vec<_>>(), width, 8)
    );

    Ok(())
}

fn get_term_width() -> Result<usize> {
    use anyhow::anyhow;

    match term_size::dimensions() {
        Some((width, _)) => Ok(width),
        None => Err(anyhow!("could not determine terminal dimensions.")),
    }
}
