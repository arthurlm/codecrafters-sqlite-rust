use crate::database::Database;

pub fn exec(db: &mut Database, index: usize) {
    let page = db.read_page(index).expect("Fail to read page");
    println!("{page:#?}");
}
