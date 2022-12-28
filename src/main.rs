use std::io::{self, Write};


fn main() -> Result<(),io::Error> {
    let buffer = "y\n".repeat(102400).into_bytes();

    let mut stdout = io::stdout().lock();
    loop {
        stdout.write_all(&buffer)?;
    }
    // Ok(())
}
