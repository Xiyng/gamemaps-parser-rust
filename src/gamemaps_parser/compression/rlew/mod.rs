#[cfg(test)]
mod tests;

extern crate byteorder;

use std::fmt;
use self::byteorder::*;

pub fn decode(data: &Vec<u8>, tag: u16, decoded_length_words: Option<usize>) -> Result<Vec<u16>, RlewDecodeError> {
    if data.len() % 2 != 0 {
        // TODO: Things seem to be working out very well even without this, so
        // it should be figured out whether this can be removed altogether.
        // return Err(RlewDecodeError::InvalidLength(data.len()));
    }

    let mut decoded = Vec::new();

    let mut words_read = 0;
    let actual_decoded_length_words = decoded_length_words.unwrap_or_else(|| {
        words_read = 1;
        LittleEndian::read_u16(&data[0..2]) as usize
    });

    while decoded.len() < actual_decoded_length_words && words_read < data.len() / 2 {
        let offset = 2 * words_read;
        let x = LittleEndian::read_u16(&data[offset..(offset + 2)]);
        let copy_wanted = x == tag;
        if !copy_wanted {
            decoded.push(x);
            words_read += 1;
            continue;
        }
        if words_read + 2 >= data.len() / 2 {
            return Err(RlewDecodeError::UnexpectedEndOfData);
        }
        let count = LittleEndian::read_u16(&data[(offset + 2)..(offset + 4)]);
        let value_to_copy = LittleEndian::read_u16(&data[(offset + 4)..(offset + 6)]);
        decoded.append(&mut vec![value_to_copy; count as usize]);
        words_read += 3;
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