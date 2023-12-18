use crate::{
    database::Database,
    error::SqliteError,
    pages::{CellArray, LeafTableCell, Page},
};

#[derive(Debug)]
pub struct CellWalker<'a> {
    database: &'a mut Database,
    page: Page,
}

impl<'a> CellWalker<'a> {
    pub fn new(database: &'a mut Database, page_index: usize) -> Result<Self, SqliteError> {
        let page = database.read_page(page_index)?;

        Ok(Self { database, page })
    }

    pub fn for_each_table_entry<F>(&'a mut self, f: F) -> Result<(), SqliteError>
    where
        F: Fn(&LeafTableCell) + Clone,
    {
        match &self.page.cells {
            // Handle leaf
            CellArray::LeafTable(cells) => {
                for cell in cells {
                    f(cell)
                }
                Ok(())
            }

            // Handle interior with recursion
            CellArray::InteriorTable(cells) => {
                for cell in cells {
                    let mut sub_walker =
                        CellWalker::new(self.database, cell.left_child_pointer - 1)?;
                    sub_walker.for_each_table_entry(f.clone())?;
                }
                Ok(())
            }

            // Ignore invalid page table type
            CellArray::InteriorIndex(_) | CellArray::LeafIndex(_) => {
                Err(SqliteError::InvalidPageType)
            }
        }
    }
}
