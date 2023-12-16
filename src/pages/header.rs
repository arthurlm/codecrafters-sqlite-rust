use nom::{
    number::complete::{be_u16, be_u32, be_u8},
    IResult,
};

use super::PageType;

#[derive(Debug)]
pub struct PageHeader {
    pub page_type: PageType,
    pub first_free_block: u16,
    pub cell_count: usize,
    pub cell_start_index: usize,
    pub fragmented_free_bytes_count: u8,
    pub right_most_pointer: Option<u32>,
}

impl PageHeader {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, page_type) = PageType::parse(input)?;
        let (input, first_free_block) = be_u16(input)?;
        let (input, cell_count) = be_u16(input)?;
        let (input, cell_start_index) = be_u16(input)?;
        let (input, fragmented_free_bytes_count) = be_u8(input)?;

        let (input, right_most_pointer) =
            if matches!(page_type, PageType::InteriorIndex | PageType::InteriorTable) {
                let (i, r) = be_u32(input)?;
                (i, Some(r))
            } else {
                (input, None)
            };

        Ok((
            input,
            Self {
                page_type,
                first_free_block,
                cell_count: cell_count as usize,
                cell_start_index: cell_start_index as usize,
                fragmented_free_bytes_count,
                right_most_pointer,
            },
        ))
    }
}
