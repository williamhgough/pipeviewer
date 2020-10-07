use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

/// CHUNK_SIZE is an arbitrary chunk size for the buffer.
const CHUNK_SIZE: usize = 16 * 1024; // 16Kb

fn main() -> Result<()> {
    // Let the user decide if progress output should show
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    // keep track of total bytes written
    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let num_read = match io::stdin().read(&mut buffer) {
            // If we get 0, we're done, break.
            Ok(0) => break,
            // If we get any other number, we're not done, return it.
            Ok(x) => x,
            // If we get an error reading, just bail for now.
            Err(_) => break,
        };
        // update total_bytes written.
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes);
        }

        // Since we're returning io::Result we could use the ? operator
        // but since we don't want to error for BrokenPipe, we have to explicitly
        // check for it and handle it.
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }

    // only print out progress is user specified they want to see it
    if !silent {
        eprintln!("\r{}", total_bytes);
    }

    Ok(())
}
