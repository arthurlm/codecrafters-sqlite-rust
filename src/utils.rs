use nom::IResult;

pub fn parse_varint(bytes: &[u8]) -> IResult<&[u8], i64> {
    let mut varint: i64 = 0;
    let mut bytes_read: usize = 0;
    for (i, byte) in bytes.iter().enumerate().take(9) {
        bytes_read += 1;
        if i == 8 {
            varint = (varint << 8) | *byte as i64;
            break;
        } else {
            varint = (varint << 7) | (*byte & 0b0111_1111) as i64;
            if *byte < 0b1000_0000 {
                break;
            }
        }
    }
    Ok((&bytes[bytes_read..], varint))
}
