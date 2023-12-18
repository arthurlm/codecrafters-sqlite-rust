use crate::{
    database::Database,
    lang::{parse_sql, SqlTree, WhereClause},
    pages::{CellArray, LeafTableCell},
    schema_def::SchemaDefinition,
};

struct ColumnInfo {
    index: usize,
    is_pk: bool,
}

fn is_included(
    cell: &LeafTableCell,
    r#where: Option<&WhereClause>,
    schema_def: &SchemaDefinition<'_>,
) -> bool {
    match r#where {
        // No filtering
        None => true,
        // Eval where clause
        Some(clause) => {
            let where_col_value = if schema_def.is_pk(&clause.column_name) {
                cell.row_id.to_string()
            } else {
                let where_col_index = schema_def
                    .column_index(&clause.column_name)
                    .expect("Invalid WHERE column name");

                cell.payload.values[where_col_index].to_string()
            };

            // FIXME: remove all this lowercase check everywhere
            clause.value.to_lowercase() == where_col_value.to_lowercase()
        }
    }
}

fn print_sql_line(cell: &LeafTableCell, column_infos: &[ColumnInfo]) {
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

pub fn exec(db: &mut Database, expression: &str) {
    // READ and check user input
    let Ok(SqlTree::Select {
        columns: column_names,
        table_name,
        r#where,
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
            if is_included(&cell, r#where.as_ref(), &schema_def) {
                print_sql_line(&cell, &column_infos);
            }
        }
    }
}
