use anyhow::Result;
use std::io;

mod columnate;
pub use columnate::columnate;

pub fn get_stdin() -> Result<String> {
    use std::io::Read;

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub fn get_term_width() -> Result<usize> {
    use anyhow::anyhow;

    match term_size::dimensions() {
        Some((width, _)) => Ok(width),
        None => Err(anyhow!("could not determine terminal dimensions.")),
    }
}

pub fn get_term_width_with_fallback(n: usize) -> usize {
    match get_term_width() {
        Ok(width) => width,
        _ => n,
    }
}
