use std::env;
use std::io::{self, Read, Write};

/// CHUNK_SIZE is an arbitrary chunk size for the buffer.
const CHUNK_SIZE: usize = 16 * 1024; // 16Kb

fn main() {
    // Let the user decide if progress output should show
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    // keep track of total bytes written
    let mut total_bytes = 0;

    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }

    // only print out progress is user specified they want to see it
    if !silent {
        eprintln!("total bytes read: {}", total_bytes);
    }
}
