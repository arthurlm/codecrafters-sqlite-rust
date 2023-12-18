use crate::{
    database::Database,
    lang::{parse_sql, SqlTree},
    pages::CellArray,
    schema_def::SchemaDefinition,
};

struct ColumnInfo {
    index: usize,
    is_pk: bool,
}

pub fn exec(db: &mut Database, expression: &str) {
    // READ and check user input
    let Ok(SqlTree::Select {
        columns: column_names,
        table_name,
        ..
    }) = parse_sql(&expression.to_lowercase())
    else {
        panic!("SQL command is not a valid SELECT statement");
    };

    // EVAL query
    // 1. Parse stored schema
    let schema_row = db
        .find_table_schema(&table_name)
        .expect("Cannot find table");

    let schema_sql = parse_sql(&schema_row.sql.to_lowercase()).expect("Fail to parse SQL schema");
    let schema_def = SchemaDefinition::try_from(&schema_sql).expect("Fail to convert SQL create");

    // 2. Find requested column index
    let column_infos: Vec<_> = column_names
        .iter()
        .map(|column_name| ColumnInfo {
            index: schema_def
                .column_index(column_name)
                .expect("Cannot find requested column in schema"),
            is_pk: schema_def.is_pk(column_name),
        })
        .collect();

    // PRINT value from SQL table.
    let root_page = db
        .read_page(schema_row.root_page - 1)
        .expect("Fail to read DB root page");

    // TODO: Handle BTree cells types
    if let CellArray::LeafTable(cells) = root_page.cells {
        for cell in cells {
            for (screen_id, column_info) in column_infos.iter().enumerate() {
                if screen_id != 0 {
                    print!("|");
                }

                if column_info.is_pk {
                    print!("{}", cell.row_id);
                } else {
                    print!("{}", cell.payload.values[column_info.index]);
                }
            }
            println!()
        }
    }
}
