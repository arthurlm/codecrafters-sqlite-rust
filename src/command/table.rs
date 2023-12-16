use crate::{database::Database, pages::CellArray, schema::SqliteSchemaRow};

pub fn exec(db: &mut Database) {
    let page = db.read_page(0).expect("Fail to read first DB page");

    if let CellArray::LeafTable(cells) = page.cells {
        for cell in cells {
            let schema = SqliteSchemaRow::parse_cell(cell).expect("Fail to read cell content");
            if schema.schema_type == "table" && !schema.name.starts_with("sqlite_") {
                print!("{} ", schema.tbl_name);
            }
        }
    }
}
