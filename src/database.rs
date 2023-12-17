use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};

use crate::{
    error::SqliteError,
    header::Header,
    pages::{CellArray, Page},
    schema::SqliteSchemaRow,
};

#[derive(Debug)]
pub struct Database {
    pub header: Header,
    reader: BufReader<File>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, SqliteError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut header_data = [0_u8; 100];
        reader.read_exact(&mut header_data)?;
        let (_, header) = Header::parse(&header_data)?;

        Ok(Self { header, reader })
    }

    pub fn read_page(&mut self, index: usize) -> Result<Page, SqliteError> {
        // Check index
        if index >= self.header.database_page_count {
            return Err(SqliteError::InvalidPageIndex);
        }

        let (first_byte_offset, first_byte) = if index == 0 {
            (Header::SIZE, Header::SIZE)
        } else {
            (0, self.header.page_size * index)
        };

        let page_size = self.header.page_size - first_byte_offset;

        // Read page content
        let mut page_data = vec![0_u8; page_size];
        self.reader.seek(SeekFrom::Start(first_byte as u64))?;
        self.reader.read_exact(&mut page_data)?;

        // Parse page content
        let page = Page::parse(&page_data, first_byte_offset)?;
        Ok(page)
    }

    pub fn schema_rows(&mut self) -> Result<Vec<SqliteSchemaRow>, SqliteError> {
        let first_page = self.read_page(0)?;
        let mut output = Vec::with_capacity(first_page.cells.len());

        if let CellArray::LeafTable(cells) = first_page.cells {
            for cell in cells {
                let schema_row = SqliteSchemaRow::parse_cell(cell)?;
                output.push(schema_row);
            }
        }

        Ok(output)
    }

    pub fn find_table_schema(&mut self, table_name: &str) -> Result<SqliteSchemaRow, SqliteError> {
        self.schema_rows()?
            .into_iter()
            .find(|x| x.tbl_name == table_name)
            .ok_or(SqliteError::TableNotFound)
    }
}
