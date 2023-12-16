mod cell;
mod cell_payload;
mod cell_value;
mod header;
mod page_type;

pub use cell::*;
pub use cell_payload::*;
pub use cell_value::*;
pub use header::*;
pub use page_type::*;

use nom::{combinator::map, multi::count, number::complete::be_u16};

use crate::error::SqliteError;

#[derive(Debug)]
pub struct Page {
    pub header: PageHeader,
    pub cells: CellArray,
}

fn parse_cell_array<O: CellParsable>(
    cell_pointers: &[usize],
    header: &PageHeader,
    page_data: &[u8],
    first_byte_offset: usize,
) -> Result<Vec<O>, SqliteError> {
    let mut output = Vec::with_capacity(cell_pointers.len());

    for cell_pointer in cell_pointers {
        assert!(
            *cell_pointer >= header.cell_start_index,
            "Invalid cell pointer: {cell_pointer}"
        );

        let cell_input = &page_data[*cell_pointer - first_byte_offset..];
        let cell_value = O::parse(cell_input)?;
        output.push(cell_value);
    }

    Ok(output)
}

impl Page {
    pub fn parse(page_data: &[u8], first_byte_offset: usize) -> Result<Self, SqliteError> {
        let (input, header) = PageHeader::parse(page_data)?;
        let (_input, cell_pointers) = count(map(be_u16, |x| x as usize), header.cell_count)(input)?;

        let cells = match header.page_type {
            PageType::InteriorIndex => CellArray::InteriorIndex(parse_cell_array(
                &cell_pointers,
                &header,
                page_data,
                first_byte_offset,
            )?),
            PageType::InteriorTable => CellArray::InteriorTable(parse_cell_array(
                &cell_pointers,
                &header,
                page_data,
                first_byte_offset,
            )?),
            PageType::LeafIndex => CellArray::LeafIndex(parse_cell_array(
                &cell_pointers,
                &header,
                page_data,
                first_byte_offset,
            )?),
            PageType::LeafTable => CellArray::LeafTable(parse_cell_array(
                &cell_pointers,
                &header,
                page_data,
                first_byte_offset,
            )?),
        };

        Ok(Self { header, cells })
    }
}
