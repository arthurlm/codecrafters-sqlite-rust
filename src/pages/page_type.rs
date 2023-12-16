use nom::{branch::alt, character::complete::char, combinator::value, IResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DatabasePageType {
    InteriorIndex,
    InteriorTable,
    LeafIndex,
    LeafTable,
}

impl DatabasePageType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            value(DatabasePageType::InteriorIndex, char(2 as char)),
            value(DatabasePageType::InteriorTable, char(5 as char)),
            value(DatabasePageType::LeafIndex, char(10 as char)),
            value(DatabasePageType::LeafIndex, char(13 as char)),
        ))(input)
    }
}
