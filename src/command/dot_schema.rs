use crate::database::Database;

pub fn exec(db: &mut Database) {
    for schema_row in db.schema_rows().expect("Fail to read DB schema") {
        if schema_row.schema_type == "table" && !schema_row.name.starts_with("sqlite_") {
            println!("{}", schema_row.sql);
        }
    }
}
