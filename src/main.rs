use std::{env, fs};

use anyhow::{bail, Result};
use sqlite_starter_rust::{command, database::Database};

fn main() -> Result<()> {
    // Parse arguments
    let db_path = env::args()
        .nth(1)
        .expect("Missing <database path> and <command>");
    let command = env::args().nth(2).expect("Missing <command>");

    // Parse command and act accordingly
    match command.as_str() {
        ".dbinfo" => {
            let data = fs::read(&db_path).expect("Fail to read file content");
            let (_, database) = Database::parse(&data).expect("Fail to read database");
            command::db_info::exec(&database);
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
