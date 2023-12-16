use nom::{bytes::complete::take, combinator::map, multi::count, IResult};

use crate::{header::DatabaseHeader, pages::DatabasePage};

#[derive(Debug)]
pub struct Database {
    pub header: DatabaseHeader,
    pages: Vec<DatabasePage>,
}

impl Database {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, header) = DatabaseHeader::parse(input)?;
        let (input, pages) = count(
            map(take(header.page_size), DatabasePage::new),
            (header.database_page_count - 1) as usize,
        )(input)?;

        Ok((input, Self { header, pages }))
    }

    pub fn page(&self, index: usize) -> Option<&DatabasePage> {
        assert!(index > 0, "Page index=0 is reserved for DatabaseHeader");
        self.pages.get(index - 1)
    }
}
