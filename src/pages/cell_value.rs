use nom::{
    bytes::complete::take,
    combinator::map,
    number::complete::{be_f64, be_i16, be_i24, be_i32, be_i64, be_i8},
    IResult,
};

#[derive(Debug)]
pub enum CellValue {
    Null,
    Number(i64),
    Float(f64),
    Blob(Vec<u8>),
    Text(String),
}

impl CellValue {
    pub fn parse(serial_type: u64, input: &[u8]) -> IResult<&[u8], Self> {
        match serial_type {
            0 => Ok((input, Self::Null)),
            // Integer
            1 => map(be_i8, |x| Self::Number(x.into()))(input),
            2 => map(be_i16, |x| Self::Number(x.into()))(input),
            3 => map(be_i24, |x| Self::Number(x.into()))(input),
            4 => map(be_i32, |x| Self::Number(x.into()))(input),
            5 => unimplemented!("Unsupported i48 bytes parse"),
            6 => map(be_i64, Self::Number)(input),
            // Float
            7 => map(be_f64, Self::Float)(input),
            // True / false with no data consume
            8 => Ok((input, Self::Number(0))),
            9 => Ok((input, Self::Number(1))),
            // Other types
            10 | 11 => unimplemented!("Unsupported internal data parse"),
            x if x % 2 == 0 => {
                let (input, content) = take(((x - 12) / 2) as usize)(input)?;
                Ok((input, Self::Blob(content.to_vec())))
            }
            x => {
                let (input, raw_content) = take(((x - 13) / 2) as usize)(input)?;
                let content = String::from_utf8_lossy(raw_content).to_string();
                Ok((input, Self::Text(content)))
            }
        }
    }
}
