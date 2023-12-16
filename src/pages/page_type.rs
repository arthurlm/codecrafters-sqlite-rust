use nom::{branch::alt, character::complete::char, combinator::value, IResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PageType {
    InteriorIndex,
    InteriorTable,
    LeafIndex,
    LeafTable,
}

impl PageType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            value(PageType::InteriorIndex, char(2 as char)),
            value(PageType::InteriorTable, char(5 as char)),
            value(PageType::LeafIndex, char(10 as char)),
            value(PageType::LeafTable, char(13 as char)),
        ))(input)
    }
}
