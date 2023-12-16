use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::char,
    combinator::{value, verify},
    number::complete::{be_u16, be_u32, be_u8},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DatabaseTextEncoding {
    Utf8,
    Utf16Le,
    Utf16Be,
}

impl DatabaseTextEncoding {
    fn read(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            value(Self::Utf8, verify(be_u32, |x| *x == 1)),
            value(Self::Utf16Le, verify(be_u32, |x| *x == 2)),
            value(Self::Utf16Be, verify(be_u32, |x| *x == 3)),
        ))(input)
    }
}

#[derive(Debug)]
pub struct DatabaseHeader {
    pub page_size: u16,
    pub write_format: u8,
    pub read_format: u8,
    pub reserved_bytes: u8,
    pub file_change_counter: u32,
    pub database_page_count: u32,
    pub first_trunk_free_list_page_number: u32,
    pub free_list_page_count: u32,
    pub schema_cookie: u32,
    pub schema_format: u32,
    pub default_cache_size: u32,
    pub text_encoding: DatabaseTextEncoding,
    pub user_version: u32,
    pub application_id: u32,
    pub software_version: u32,
}

impl DatabaseHeader {
    pub fn read(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, _) = tag(b"SQLite format 3\0")(input)?;
        let (input, page_size) = be_u16(input)?;
        let (input, write_format) = be_u8(input)?;
        let (input, read_format) = be_u8(input)?;
        let (input, reserved_bytes) = be_u8(input)?;
        let (input, _max_payload_frac) = char(64 as char)(input)?;
        let (input, _min_payload_frac) = char(32 as char)(input)?;
        let (input, _leaf_payload_frac) = char(32 as char)(input)?;
        let (input, file_change_counter) = be_u32(input)?;
        let (input, database_page_count) = be_u32(input)?;
        let (input, first_trunk_free_list_page_number) = be_u32(input)?;
        let (input, free_list_page_count) = be_u32(input)?;
        let (input, schema_cookie) = be_u32(input)?;
        let (input, schema_format) = be_u32(input)?;
        let (input, default_cache_size) = be_u32(input)?;
        let (input, _) = be_u32(input)?;
        let (input, text_encoding) = DatabaseTextEncoding::read(input)?;
        let (input, user_version) = be_u32(input)?;
        let (input, _) = be_u32(input)?;
        let (input, application_id) = be_u32(input)?;
        let (input, _) = take(20_usize)(input)?;
        let (input, _) = be_u32(input)?;
        let (input, software_version) = be_u32(input)?;

        Ok((
            input,
            Self {
                page_size,
                write_format,
                read_format,
                reserved_bytes,
                file_change_counter,
                database_page_count,
                first_trunk_free_list_page_number,
                free_list_page_count,
                schema_cookie,
                schema_format,
                default_cache_size,
                text_encoding,
                user_version,
                application_id,
                software_version,
            },
        ))
    }
}
