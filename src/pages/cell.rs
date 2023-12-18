use nom::{bytes::complete::take, number::complete::be_u32};

use crate::{error::SqliteError, utils::parse_varint};

use super::CellPayload;

#[derive(Debug)]
pub enum CellArray {
    InteriorIndex(Vec<InteriorIndexCell>),
    InteriorTable(Vec<InteriorTableCell>),
    LeafIndex(Vec<LeafIndexCell>),
    LeafTable(Vec<LeafTableCell>),
}

#[derive(Debug)]
pub struct InteriorIndexCell {
    pub left_child_pointer: usize,
    pub payload: CellPayload,
}

#[derive(Debug)]
pub struct InteriorTableCell {
    pub left_child_pointer: usize,
    pub row_id: i64,
}

#[derive(Debug)]
pub struct LeafIndexCell {
    pub payload: CellPayload,
}

#[derive(Debug)]
pub struct LeafTableCell {
    pub row_id: i64,
    pub payload: CellPayload,
}

impl CellArray {
    pub fn len(&self) -> usize {
        match self {
            CellArray::InteriorIndex(x) => x.len(),
            CellArray::InteriorTable(x) => x.len(),
            CellArray::LeafIndex(x) => x.len(),
            CellArray::LeafTable(x) => x.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait CellParsable {
    fn parse(input: &[u8]) -> Result<Self, SqliteError>
    where
        Self: Sized;
}

impl CellParsable for InteriorIndexCell {
    fn parse(input: &[u8]) -> Result<Self, SqliteError> {
        let (input, left_child_pointer) = be_u32(input)?;
        let (input, payload_size) = parse_varint(input)?;
        let (_, raw_payload) = take(payload_size as usize)(input)?;
        let payload = CellPayload::parse(raw_payload)?;

        Ok(Self {
            left_child_pointer: left_child_pointer as usize,
            payload,
        })
    }
}

impl CellParsable for InteriorTableCell {
    fn parse(input: &[u8]) -> Result<Self, SqliteError> {
        let (input, left_child_pointer) = be_u32(input)?;
        let (_, row_id) = parse_varint(input)?;
        Ok(Self {
            left_child_pointer: left_child_pointer as usize,
            row_id,
        })
    }
}

impl CellParsable for LeafIndexCell {
    fn parse(input: &[u8]) -> Result<Self, SqliteError> {
        let (input, payload_size) = parse_varint(input)?;
        let (_, raw_payload) = take(payload_size as usize)(input)?;
        let payload = CellPayload::parse(raw_payload)?;

        Ok(Self { payload })
    }
}

impl CellParsable for LeafTableCell {
    fn parse(input: &[u8]) -> Result<Self, SqliteError> {
        let (input, payload_size) = parse_varint(input)?;
        let (input, row_id) = parse_varint(input)?;
        let (_, raw_payload) = take(payload_size as usize)(input)?;
        let payload = CellPayload::parse(raw_payload)?;

        Ok(Self { row_id, payload })
    }
}
