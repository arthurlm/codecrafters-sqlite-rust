use crate::database::Database;

pub fn exec(db: &mut Database) {
    for schema_row in db.schema_rows().expect("Fail to read DB schema") {
        println!("{}", schema_row.sql);
    }
}
