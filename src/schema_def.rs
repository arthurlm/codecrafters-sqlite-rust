use std::collections::HashMap;

use crate::{error::SqliteError, lang::SqlTree};

#[derive(Debug, PartialEq, Eq)]
pub struct SchemaDefinition<'a> {
    pub table_name: &'a str,
    pub pk_index: usize,
    pub column_indexes: HashMap<&'a str, usize>,
}

impl<'a> TryFrom<&'a SqlTree> for SchemaDefinition<'a> {
    type Error = SqliteError;

    fn try_from(sql_tree: &'a SqlTree) -> Result<Self, Self::Error> {
        let SqlTree::CreateTable {
            table_name,
            columns_def,
        } = sql_tree
        else {
            return Err(SqliteError::InvalidSqliteSchema);
        };

        let pk_index = columns_def
            .iter()
            .position(|x| x.is_primary_key)
            .ok_or(SqliteError::MissingPrimaryKey)?;

        let column_indexes = columns_def
            .iter()
            .enumerate()
            .map(|(idx, col)| (col.name.as_str(), idx))
            .collect();

        Ok(Self {
            table_name,
            pk_index,
            column_indexes,
        })
    }
}

impl SchemaDefinition<'_> {
    pub fn is_pk(&self, column_name: &str) -> bool {
        matches!(self.column_indexes.get(column_name) , Some(index) if *index == self.pk_index )
    }

    pub fn column_index(&self, column_name: &str) -> Option<usize> {
        self.column_indexes.get(column_name).copied()
    }
}
