#[cfg(test)]
mod tests;

extern crate byteorder;

use std::fmt;
use self::byteorder::*;

const NEAR_SIGNAL: u8 = 0xa7;
const FAR_SIGNAL: u8 = 0xa8;

pub fn decompress(data: &Vec<u8>, start_offset: usize) -> Result<Vec<u16>, DecompressionError> {
    let decompressed_length_bytes = LittleEndian::read_u16(&data[start_offset..(start_offset + 2)]) as usize;
    let decompressed_length_words = decompressed_length_bytes / 2;
    let mut decompressed = Vec::new();

    let mut i  = start_offset + 2;
    while decompressed.len() < decompressed_length_words && i < data.len() {
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

                let offset = data[i] as usize;
                let repeat_start = decompressed.len() - offset;
                let repeat_end = repeat_start + (current as usize);
                if repeat_end > decompressed.len() {
                    return Err(DecompressionError::NearPointerOffsetOutOfBounds {
                        decompressed_length: decompressed.len(),
                        offset: offset as u8,
                        word_count: current
                    })
                }
                let mut words = decompressed[repeat_start..repeat_end].to_vec();
                decompressed.append(&mut words);
                i += 1;
            },
            FAR_SIGNAL => {
                if i + 1 >= data.len() {
                    return Err(DecompressionError::InvalidLength(data.len()));
                }

                let offset = LittleEndian::read_u16(&data[i..(i + 2)]) as usize;
                let repeat_start = offset;
                let repeat_end = offset + (current as usize);
                if repeat_end > decompressed.len() {
                    return Err(DecompressionError::FarPointerOffsetOutOfBounds {
                        decompressed_length: decompressed.len(),
                        offset: offset as u16,
                        word_count: current
                    })
                }
                let mut words = decompressed[repeat_start..repeat_end].to_vec();
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
    NearPointerOffsetOutOfBounds {
        decompressed_length: usize,
        offset: u8,
        word_count: u8
    },
    FarPointerOffsetOutOfBounds {
        decompressed_length: usize,
        offset: u16,
        word_count: u8
    }
}

impl fmt::Display for DecompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecompressionError::InvalidLength(length) =>
                write!(f, "Invalid length: {}", length),
            DecompressionError::NearPointerOffsetOutOfBounds { decompressed_length, offset, word_count } =>
                write!(f,
                "Near pointer decompression out of bounds after {} decompressed words: 0x{:x?} words to decompress with an offset of 0x{:x?} words",
                decompressed_length, word_count, offset
            ),
            DecompressionError::FarPointerOffsetOutOfBounds { decompressed_length, offset, word_count } =>
                write!(f,
                "Far pointer decompression out of bounds after {} decompressed words: 0x{:x?} words to decompress with an offset of 0x{:x?} words",
                decompressed_length, word_count, offset
            )
        }
    }
}