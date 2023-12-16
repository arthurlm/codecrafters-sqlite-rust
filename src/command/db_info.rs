use crate::header::DatabaseHeader;

pub fn exec(header: &DatabaseHeader) {
    println!("database page size: {}", header.page_size);
    println!("write format:       {}", header.write_format);
    println!("read format:        {}", header.read_format);
    println!("reserved bytes:     {}", header.reserved_bytes);
    println!("file change counter:{}", header.file_change_counter);
    println!("database page count:{}", header.database_page_count);
    println!("freelist page count:{}", header.free_list_page_count);
    println!("schema cookie:      {}", header.schema_cookie);
    println!("schema format:      {}", header.schema_format);
    println!("default cache size: {}", header.default_cache_size);
    // println!("autovacuum top root:{}", self.autovacuum_top_root);
    // println!("incremental vacuum: {}", 0);
    println!("text encoding:      {:?}", header.text_encoding);
    println!("user version:       {}", header.user_version);
    println!("application id:     {}", header.application_id);
    println!("software version:   {}", header.software_version);
    // println!("number of tables:   {}", 3);
    // println!("number of indexes:  {}", 0);
    // println!("number of triggers: {}", 0);
    // println!("number of views:    {}", 0);
    // println!("schema size:        {}", 217);
    // println!("data version        {}", 1);
}
