use std::io::{Read, Write};
use std::{env, io};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    let silent = !env::var("PV_SILENT").unwrap_or(String::new()).is_empty();
    dbg!(silent);
    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(num) => num,
            Err(_) => break,
        };
        total_bytes += num_read;
        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }
    eprintln!("total_bytes:{}", total_bytes);
}
