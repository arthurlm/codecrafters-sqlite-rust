mod header;
mod page_type;

pub use header::*;
pub use page_type::*;

#[derive(Debug)]
pub struct DatabasePage {
    pub page_data: Vec<u8>,
}

impl DatabasePage {
    pub fn new(page_data: &[u8]) -> Self {
        Self {
            page_data: page_data.to_vec(),
        }
    }

    pub fn parse_header(&self) -> Option<DatabasePageHeader> {
        let (_, output) = DatabasePageHeader::parse(&self.page_data).ok()?;
        Some(output)
    }
}
