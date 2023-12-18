use crate::{
    database::Database,
    lang::{is_index_for, parse_sql, SqlTree, WhereClause},
    pages::{CellValue, LeafTableCell},
    schema_def::SchemaDefinition,
    walker::CellWalker,
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
        Some(WhereClause { column_name, value }) => {
            if schema_def.is_pk(column_name) {
                cell.row_id.to_string() == *value
            } else {
                let where_col_index = schema_def
                    .column_index(column_name)
                    .expect("Invalid WHERE column name");

                cell.payload.values[where_col_index].to_string() == *value
            }
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
    }) = parse_sql(expression)
    else {
        panic!("SQL command is not a valid SELECT statement");
    };

    // EVAL query
    // 1. Parse stored schema
    let schema_row = db
        .find_table_schema(&table_name)
        .expect("Cannot find table");

    let schema_sql = parse_sql(&schema_row.sql).expect("Fail to parse SQL schema");
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

    // 3. Check if there is a where clause with some index in it.
    let row_ids_opt = index_full_scan(db, &table_name, r#where.as_ref());

    // PRINT value from SQL table.
    if let Some(row_ids) = row_ids_opt {
        for row_id in row_ids {
            CellWalker::new(db)
                .find(schema_row.root_page - 1, row_id, |cell| {
                    if is_included(cell, r#where.as_ref(), &schema_def) {
                        print_sql_line(cell, &column_infos);
                    }
                })
                .expect("Fail to walk table entries");
        }
    } else {
        CellWalker::new(db)
            .for_each_table_entry(schema_row.root_page - 1, |cell| {
                if is_included(cell, r#where.as_ref(), &schema_def) {
                    print_sql_line(cell, &column_infos);
                }
            })
            .expect("Fail to walk table entries");
    }
}

fn index_full_scan(
    db: &mut Database,
    target_table_name: &str,
    r#where: Option<&WhereClause>,
) -> Option<Vec<i64>> {
    // Early exit if there is no where filter
    let Some(WhereClause {
        column_name: where_column_name,
        value: where_column_value,
        ..
    }) = r#where.as_ref()
    else {
        return None;
    };

    // Try to find an index for the requested column
    let index_schema_rows = db
        .find_table_indexes(target_table_name)
        .expect("Fail to read DB schema indexes");

    let index_root_page = index_schema_rows
        .iter()
        .map(|index_schema_row| {
            (
                index_schema_row.root_page,
                parse_sql(&index_schema_row.sql).expect("Fail to parse index"),
            )
        })
        .filter(|(_root_page, sql)| is_index_for(sql, target_table_name, &[where_column_name]))
        .map(|(root_page, _sql)| root_page)
        .next()?;

    // Walk the whole index and search for row IDs.
    let row_ids = CellWalker::new(db)
        .for_each_index_entry(index_root_page - 1, |cell| {
            if let [CellValue::Text(col), CellValue::Number(row_id)] = &cell.payload.values[..] {
                if *col == *where_column_value {
                    return Some(*row_id);
                }
            }

            None
        })
        .expect("Fail to walk index");

    Some(row_ids.into_iter().filter_map(|x| x).collect())
}
