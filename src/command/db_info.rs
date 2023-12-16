use crate::database::Database;

pub fn exec(db: &Database) {
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

    let first_page = db.page(1).expect("Fail to read first page");
    let first_page_header = first_page.parse_header().expect("Fail to read page header");
    // NOTE: Bellow result if a little bit wrong sqlite_schema can contains other things than table
    println!("number of tables:   {}", first_page_header.cell_count);
    // println!("number of indexes:  {}", 0);
    // println!("number of triggers: {}", 0);
    // println!("number of views:    {}", 0);
    // println!("schema size:        {}", 217);
    // println!("data version        {}", 1);
}
