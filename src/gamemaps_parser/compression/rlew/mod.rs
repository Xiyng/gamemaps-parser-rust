#[cfg(test)]
mod tests;

extern crate byteorder;

use std::fmt;
use self::byteorder::*;

pub fn decode(data: &Vec<u8>, tag: u16) -> Result<Vec<u16>, RlewDecodeError> {
    if data.len() % 2 != 0 {
        return Err(RlewDecodeError::InvalidLength(data.len()));
    }

    let mut decoded = Vec::new();
    let decoded_length_bytes = LittleEndian::read_u32(&data[0..4]) as usize;
    let mut i = 4;
    while decoded.len() < 2 * decoded_length_bytes && i < data.len() / 2 {
        let offset = 2 * i;
        let x = LittleEndian::read_u16(&data[offset..(offset + 2)]);
        let copy_wanted = x == tag;
        if !copy_wanted {
            decoded.push(x);
            i += 1;
            continue;
        }
        if i + 2 >= data.len() / 2 {
            return Err(RlewDecodeError::UnexpectedEndOfData);
        }
        let count = LittleEndian::read_u16(&data[(offset + 2)..(offset + 4)]);
        let value_to_copy = LittleEndian::read_u16(&data[(offset + 4)..(offset + 6)]);
        decoded.append(&mut vec![value_to_copy; count as usize]);
        i += 3;
    }

    Ok(decoded)
}

#[derive(Debug, PartialEq)]
pub enum RlewDecodeError {
    InvalidLength(usize),
    UnexpectedEndOfData
}

impl fmt::Display for RlewDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RlewDecodeError::InvalidLength(length) =>
                write!(f, "Invalid length: {}", length),
            RlewDecodeError::UnexpectedEndOfData =>
                write!(f, "Unexpected end of data")
        }
    }
}