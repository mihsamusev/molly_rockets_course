use std::borrow::Borrow;
use std::io::Read;
use std::fmt::Write;
use std::{fs, io};


pub fn read_bytes(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(filename)?;
    let metadata = fs::metadata(filename)?;
    let mut buffer = vec![0; metadata.len() as usize];
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}

pub fn read_bytes_cli() -> Result<Vec<u8>, String> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => Err("Have not found binary to decompile".into()),
        _ => {
            let filename = args[1].borrow();
            read_bytes(filename).map_err(|_| format!("Unable to read file '{}'", filename))
        }
    }
}


pub fn format_bytes(bytes: &[u8], start: usize, end: usize) -> String {
    let mut result = String::new();
    for byte in bytes[start..end].iter() {
        write!(result, "[{:#o}]", byte).expect("unable to display byte");
    }
    result.push('\n');
    result
}