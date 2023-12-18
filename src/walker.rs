use crate::{
    database::Database,
    error::SqliteError,
    pages::{CellArray, LeafTableCell},
};

#[derive(Debug)]
pub struct CellWalker<'a> {
    database: &'a mut Database,
}

impl<'a> CellWalker<'a> {
    pub fn new(database: &'a mut Database) -> Self {
        Self { database }
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

            // Ignore invalid page table type
            CellArray::InteriorIndex(_) | CellArray::LeafIndex(_) => {
                Err(SqliteError::InvalidPageType)
            }
        }
    }
}
