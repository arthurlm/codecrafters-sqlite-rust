use anyhow::{bail, Result};
use std::{env, fs::File, io::Read};

fn main() -> Result<()> {
    // Parse arguments
    let db_path = env::args()
        .nth(1)
        .expect("Missing <database path> and <command>");
    let command = env::args().nth(2).expect("Missing <command>");

    // Parse command and act accordingly
    match command.as_str() {
        ".dbinfo" => {
            let mut file = File::open(&db_path)?;
            let mut header = [0; 100];
            file.read_exact(&mut header)?;

            // The page size is stored at the 16th byte offset, using 2 bytes in big-endian order
            #[allow(unused_variables)]
            let page_size = u16::from_be_bytes([header[16], header[17]]);

            // Uncomment this block to pass the first stage
            println!("database page size: {page_size}");
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
