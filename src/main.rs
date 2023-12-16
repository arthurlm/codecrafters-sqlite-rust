use std::env;

use anyhow::{bail, Result};
use sqlite_starter_rust::{command, database::Database};

fn main() -> Result<()> {
    // Parse arguments
    let db_path = env::args()
        .nth(1)
        .expect("Missing <database path> and <command>");
    let command = env::args().nth(2).expect("Missing <command>");
    let mut database = Database::open(db_path).expect("Fail to open database");

    // Parse command and act accordingly
    match command.as_str() {
        ".dbinfo" => {
            command::db_info::exec(&mut database);
        }
        ".tables" => {
            command::table::exec(&mut database);
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
