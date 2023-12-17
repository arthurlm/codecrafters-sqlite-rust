use crate::database::Database;

pub fn exec(db: &mut Database, table_name: &str) {
    let schema = db.find_table_schema(table_name).expect("Table not found");
    let root_page = db
        .read_page(schema.root_page - 1)
        .expect("Fail to read root page");

    println!("{}", root_page.cells.len());
}
