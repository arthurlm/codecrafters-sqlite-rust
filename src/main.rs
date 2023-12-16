use anyhow::{bail, Result};
use sqlite_starter_rust::{command, header::DatabaseHeader};
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
            let mut raw_header = [0; 100];
            file.read_exact(&mut raw_header)?;

            let (_, db_header) = DatabaseHeader::read(&raw_header).expect("Fail to read header");
            command::db_info::exec(&db_header);
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
