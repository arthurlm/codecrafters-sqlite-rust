use nom::{
    number::complete::{be_u16, be_u8},
    IResult,
};

use super::DatabasePageType;

#[derive(Debug)]
pub struct DatabasePageHeader {
    pub page_type: DatabasePageType,
    pub first_free_block: u16,
    pub cell_count: u16,
    pub cell_start_index: u16,
    pub fragmented_free_bytes_count: u8,
}

impl DatabasePageHeader {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, page_type) = DatabasePageType::parse(input)?;
        let (input, first_free_block) = be_u16(input)?;
        let (input, cell_count) = be_u16(input)?;
        let (input, cell_start_index) = be_u16(input)?;
        let (input, fragmented_free_bytes_count) = be_u8(input)?;
        Ok((
            input,
            Self {
                page_type,
                first_free_block,
                cell_count,
                cell_start_index,
                fragmented_free_bytes_count,
            },
        ))
    }
}
