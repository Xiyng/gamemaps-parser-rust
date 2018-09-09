#[cfg(test)]
mod tests;

extern crate byteorder;

// use std::fmt;
use self::byteorder::*;

pub fn decompress(data: &Vec<u8>) -> Result<Vec<u16>, DecompressionError> {
    let mut decompressed = Vec::new();

    let mut i  = 0;
    while i < data.len() {
        if i + 1 >= data.len() {
            return Err(DecompressionError::InvalidLength(data.len()));
        }

        let current = data[i];
        let next = data[i + 1];
        i += 2;

        if current == 0x00 && (next == 0xa7 || next == 0xa8) {
            if i >= data.len() {
                return Err(DecompressionError::InvalidLength(data.len()));
            }
            let second_byte = data[i];
            decompressed.push(LittleEndian::read_u16(&vec![next, second_byte]));
            i += 1;
            continue;
        }

        match next {
            0xa7 => {
                if i >= data.len() {
                    return Err(DecompressionError::InvalidLength(data.len()));
                }

                let repeat_offset = 2 * data[i]; // as words, not bytes
                let repeat_start = i - repeat_offset as usize; // TODO: Check for range.
                let mut words = vec![0; current as usize];
                LittleEndian::read_u16_into(
                    &data[repeat_start..repeat_start + (2 * current) as usize],
                    &mut words
                );
                decompressed.append(&mut words);
                i += 1;
            },
            0xa8 => {
                if i + 1 >= data.len() {
                    return Err(DecompressionError::InvalidLength(data.len()));
                }

                let repeat_start = 2 * LittleEndian::read_u16(&data[i..(i + 2)]) as usize;
                let mut words = vec![0; current as usize];
                LittleEndian::read_u16_into(
                    &data[repeat_start..repeat_start + (2 * current) as usize],
                    &mut words
                );
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
    InvalidLength(usize)
}