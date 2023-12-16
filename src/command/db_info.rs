use crate::{database::Database, pages::CellArray, schema::SqliteSchemaRow};

pub fn exec(db: &mut Database) {
    // Show global value configuration
    println!("database page size: {}", db.header.page_size);
    println!("write format:       {}", db.header.write_format);
    println!("read format:        {}", db.header.read_format);
    println!("reserved bytes:     {}", db.header.reserved_bytes);
    println!("file change counter:{}", db.header.file_change_counter);
    println!("database page count:{}", db.header.database_page_count);
    println!("freelist page count:{}", db.header.free_list_page_count);
    println!("schema cookie:      {}", db.header.schema_cookie);
    println!("schema format:      {}", db.header.schema_format);
    println!("default cache size: {}", db.header.default_cache_size);
    // println!("autovacuum top root:{}", 0);
    // println!("incremental vacuum: {}", 0);
    println!("text encoding:      {:?}", db.header.text_encoding);
    println!("user version:       {}", db.header.user_version);
    println!("application id:     {}", db.header.application_id);
    println!("software version:   {}", db.header.software_version);

    // Show schema configuration
    let first_page = db.read_page(0).expect("Fail to read first page");

    let mut table_count = 0;
    let mut index_count = 0;
    let mut triggers_count = 0;
    let mut view_count = 0;

    if let CellArray::LeafTable(cells) = first_page.cells {
        for cell in cells {
            let schema = SqliteSchemaRow::parse_cell(cell).expect("Fail to read cell content");
            match schema.schema_type.as_str() {
                "table" => table_count += 1,
                "index" => index_count += 1,
                "trigger" => triggers_count += 1,
                "view" => view_count += 1,
                _ => {}
            }
        }
    }

    println!("number of tables:   {}", table_count);
    println!("number of indexes:  {}", index_count);
    println!("number of triggers: {}", triggers_count);
    println!("number of views:    {}", view_count);

    // println!("schema size:        {}", 217);
    // println!("data version        {}", 1);
}
