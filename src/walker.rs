use crate::{
    database::Database,
    error::SqliteError,
    pages::{CellArray, LeafIndexCell, LeafTableCell},
};

#[derive(Debug)]
pub struct CellWalker<'a> {
    database: &'a mut Database,
}

impl<'a> CellWalker<'a> {
    pub fn new(database: &'a mut Database) -> Self {
        Self { database }
    }

    pub fn find<F, O>(&'a mut self, page_index: usize, row_id: i64, f: F) -> Result<O, SqliteError>
    where
        F: Fn(&LeafTableCell) -> O + Clone,
    {
        let page = self.database.read_page(page_index)?;

        match (&page.cells, page.header.right_most_pointer) {
            // Handle leaf
            (CellArray::LeafTable(cells), _) => {
                for cell in cells {
                    if cell.row_id == row_id {
                        return Ok(f(cell));
                    }
                }

                Err(SqliteError::RowIdNotFound)
            }

            // Handle interior with recursion
            (CellArray::InteriorTable(cells), Some(rmp)) => {
                for cell in cells {
                    if row_id <= cell.row_id {
                        let output = CellWalker::new(self.database).find(
                            cell.left_child_pointer - 1,
                            row_id,
                            f.clone(),
                        )?;

                        return Ok(output);
                    }
                }

                let output =
                    CellWalker::new(self.database).find(rmp as usize - 1, row_id, f.clone())?;

                return Ok(output);
            }

            // Ignore invalid page type
            _ => Err(SqliteError::InvalidPageType),
        }
    }

    pub fn for_each_table_entry<F, O>(
        &'a mut self,
        page_index: usize,
        f: F,
    ) -> Result<Vec<O>, SqliteError>
    where
        F: Fn(&LeafTableCell) -> O + Clone,
    {
        let page = self.database.read_page(page_index)?;

        match &page.cells {
            // Handle leaf
            CellArray::LeafTable(cells) => Ok(cells.iter().map(f).collect()),

            // Handle interior with recursion
            CellArray::InteriorTable(cells) => {
                let mut output = Vec::new();
                for cell in cells {
                    let output_chunk = CellWalker::new(self.database)
                        .for_each_table_entry(cell.left_child_pointer - 1, f.clone())?;

                    output.extend(output_chunk);
                }
                Ok(output)
            }

            // Ignore invalid page type
            CellArray::InteriorIndex(_) | CellArray::LeafIndex(_) => {
                Err(SqliteError::InvalidPageType)
            }
        }
    }

    pub fn for_each_index_entry<F, O>(
        &'a mut self,
        page_index: usize,
        f: F,
    ) -> Result<Vec<O>, SqliteError>
    where
        F: Fn(&LeafIndexCell) -> O + Clone,
    {
        let page = self.database.read_page(page_index)?;

        match &page.cells {
            // Handle leaf
            CellArray::LeafIndex(cells) => Ok(cells.iter().map(f).collect()),

            // Handle interior with recursion
            CellArray::InteriorIndex(cells) => {
                let mut output = Vec::new();
                for cell in cells {
                    let output_chunk = CellWalker::new(self.database)
                        .for_each_index_entry(cell.left_child_pointer - 1, f.clone())?;

                    output.extend(output_chunk);
                }
                Ok(output)
            }

            // Ignore invalid page type
            CellArray::InteriorTable(_) | CellArray::LeafTable(_) => {
                Err(SqliteError::InvalidPageType)
            }
        }
    }
}
