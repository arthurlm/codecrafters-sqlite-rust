use crate::{
    error::SqliteError,
    pages::{CellValue, LeafTableCell},
};

#[derive(Debug)]
pub struct SqliteSchemaRow {
    pub schema_type: String,
    pub name: String,
    pub tbl_name: String,
    pub root_page: usize,
    pub sql: String,
}

impl SqliteSchemaRow {
    pub fn parse_cell(cell: LeafTableCell) -> Result<Self, SqliteError> {
        let mut cell_values = cell.payload.values.into_iter();

        let Some(CellValue::Text(schema_type)) = cell_values.next() else {
            return Err(SqliteError::InvalidSqliteSchema);
        };
        let Some(CellValue::Text(name)) = cell_values.next() else {
            return Err(SqliteError::InvalidSqliteSchema);
        };
        let Some(CellValue::Text(tbl_name)) = cell_values.next() else {
            return Err(SqliteError::InvalidSqliteSchema);
        };
        let Some(CellValue::Number(root_page)) = cell_values.next() else {
            return Err(SqliteError::InvalidSqliteSchema);
        };
        let Some(CellValue::Text(sql)) = cell_values.next() else {
            return Err(SqliteError::InvalidSqliteSchema);
        };

        Ok(Self {
            schema_type,
            name,
            tbl_name,
            root_page: root_page as usize,
            sql,
        })
    }
}
