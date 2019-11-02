use std::io;

mod columnate;
pub use columnate::columnate;

pub fn get_stdin() -> io::Result<String> {
    use std::io::Read;

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    Ok(buffer)
}
