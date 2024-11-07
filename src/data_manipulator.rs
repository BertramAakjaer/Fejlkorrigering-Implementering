use std::fs::File;
use std::io::{self, Write};

pub fn save_as_binary(data: &str, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
