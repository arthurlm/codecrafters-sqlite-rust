use crate::{error::SqliteError, utils::parse_varint};

use super::CellValue;

#[derive(Debug)]
pub struct CellPayload {
    pub values: Vec<CellValue>,
}

impl CellPayload {
    pub fn parse(input: &[u8]) -> Result<Self, SqliteError> {
        // Parse header
        let (input, header_size) = parse_varint(input)?;
        let (mut header_input, mut data_input) =
            input.split_at((header_size as usize).saturating_sub(1));

        let mut column_types = Vec::new();
        while !header_input.is_empty() {
            let (next_header_data, value) = parse_varint(header_input)?;

            column_types.push(value);
            header_input = next_header_data;
        }

        // Parse data
        let mut values = Vec::with_capacity(column_types.len());
        for column_type in column_types {
            let (next_data_input, value) = CellValue::parse(column_type as u64, data_input)?;

            values.push(value);
            data_input = next_data_input;
        }

        Ok(Self { values })
    }
}
