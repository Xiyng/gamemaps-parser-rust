#[cfg(test)]
mod tests;

extern crate byteorder;

use self::byteorder::*;

pub fn decode(data: &Vec<u8>) -> Result<Vec<u16>, RlewDecodeError> {
    if data.len() % 4 != 0 {
        return Err(RlewDecodeError::InvalidLength(data.len()));
    }

    let mut decoded = Vec::new();
    for i in 0..data.len() / 4 {
        let offset = 4 * i;
        let x = LittleEndian::read_u16(&data[offset..(offset + 2)]);
        let count = LittleEndian::read_u16(&data[(offset + 2)..(offset + 4)]);
        decoded.append(&mut vec![x; count as usize]);
    }

    Ok(decoded)
}

#[derive(Debug, PartialEq)]
pub enum RlewDecodeError {
    InvalidLength(usize)
}