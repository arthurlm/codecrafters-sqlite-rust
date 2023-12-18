use crate::{
    database::Database,
    lang::{parse_sql, SqlTree},
    pages::CellArray,
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

    let Ok(SqlTree::CreateTable { columns_def, .. }) = parse_sql(&schema_row.sql.to_lowercase())
    else {
        panic!("Cannot parse SQL schema");
    };

    // 2. Find requested column index
    let mut column_infos = Vec::with_capacity(column_names.len());
    for column_name in column_names {
        let index = columns_def
            .iter()
            .position(|c| c.name == column_name)
            .expect("Cannot find requested column in schema");

        column_infos.push(ColumnInfo {
            index,
            is_pk: columns_def[index].is_primary_key,
        });
    }

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
