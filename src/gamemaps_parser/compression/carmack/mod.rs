#[cfg(test)]
mod tests;

extern crate byteorder;

use std::fmt;
use self::byteorder::*;

const NEAR_SIGNAL: u8 = 0xa7;
const FAR_SIGNAL: u8 = 0xa8;

pub fn decompress(data: &Vec<u8>, start_offset: usize) -> Result<Vec<u16>, DecompressionError> {
    let decompressed_length_bytes = LittleEndian::read_u16(&data[start_offset..(start_offset + 2)]) as usize;
    let mut decompressed = Vec::new();

    let mut i  = start_offset + 2;
    while decompressed.len() < 2 * decompressed_length_bytes && i < data.len() {
        if i + 1 >= data.len() {
            return Err(DecompressionError::InvalidLength(data.len()));
        }

        let current = data[i];
        let next = data[i + 1];
        i += 2;

        if current == 0x00 && (next == NEAR_SIGNAL || next == FAR_SIGNAL) {
            if i >= data.len() {
                return Err(DecompressionError::InvalidLength(data.len()));
            }
            let second_byte = data[i];
            decompressed.push(LittleEndian::read_u16(&vec![next, second_byte]));
            i += 1;
            continue;
        }

        match next {
            NEAR_SIGNAL => {
                if i >= data.len() {
                    return Err(DecompressionError::InvalidLength(data.len()));
                }

                let repeat_start = decompressed.len() - 1;
                let mut words = decompressed[repeat_start..repeat_start + (current as usize)].to_vec();
                decompressed.append(&mut words);
                i += 1;
            },
            FAR_SIGNAL => {
                if i + 1 >= data.len() {
                    return Err(DecompressionError::InvalidLength(data.len()));
                }

                let repeat_start = LittleEndian::read_u16(&data[i..(i + 2)]) as usize;
                let mut words = decompressed[repeat_start..repeat_start + (current as usize)].to_vec();
                decompressed.append(&mut words);
                i += 2;
            },
            _ => decompressed.push(LittleEndian::read_u16(&vec![current, next]))
        }
    }

    Ok(decompressed)
}

#[derive(Debug, PartialEq)]
pub enum DecompressionError {
    InvalidLength(usize),
    InvalidNearPointerOffset {
        index: usize,
        offset: u8
    }
}

impl fmt::Display for DecompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecompressionError::InvalidLength(length) =>
                write!(f, "Invalid length: {}", length),
            DecompressionError::InvalidNearPointerOffset { index, offset } =>
                write!(f, "Invalid near pointer offset for index 0x{:x?}: 0x{:x?} words", index, offset)
        }
    }
}