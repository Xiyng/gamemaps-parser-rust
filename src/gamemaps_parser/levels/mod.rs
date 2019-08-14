#[cfg(test)]
mod tests;

extern crate byteorder;

use std::ffi::CStr;
use std::fmt;
use gamemaps_parser::compression::{carmack, rlew};
use self::byteorder::*;

const RLEW_TAG: u16 = 0xabcd;
const HEADER_LENGTH_U8: usize = 42;

pub fn parse(data: &Vec<u8>, offset: u32) -> Result<Level, LevelParseError> {
    validate_magic_str(&data)?;

    let level_header = parse_level_header(&data, offset as usize, HEADER_LENGTH_U8 / 2)?;
    let plane_count = level_header.plane_headers.len();
    let mut planes = Vec::with_capacity(plane_count);
    let mut i = 0;
    for plane_header in level_header.plane_headers.iter() {
        let carmack_decompressed_data = carmack::decompress(&data, plane_header.offset as usize).map_err(|e|
            LevelParseError::CarmackDecompressionError {
                plane: i,
                error: e
            }
        )?;
        let mut carmack_decompressed_data_u8 = vec![0; 2 * carmack_decompressed_data.len()];
        LittleEndian::write_u16_into(
            &carmack_decompressed_data,
            &mut carmack_decompressed_data_u8
        );
        let rlew_decoded_data = rlew::decode(&carmack_decompressed_data_u8, RLEW_TAG, None).map_err(|e|
            LevelParseError::PlaneRlewDecodeError {
                plane: i,
                error: e
            }
        )?;

        // TODO: Do Carmack decompression only when it's needed.
        planes.push(Plane { data: rlew_decoded_data });

        i += 1;
    }

    Ok(Level {
        name: level_header.name,
        width: level_header.width,
        height: level_header.height,
        planes: planes
    })
}

fn validate_magic_str(data: &Vec<u8>) -> Result<(), LevelParseError> {
    let magic_str = "TED5v1.0";

    if data.len() < magic_str.len() {
        return Err(LevelParseError::UnexpectedEndOfData);
    }

    let actual_str = unsafe {
        String::from_utf8_unchecked(data[0..magic_str.len()].to_vec())
    };
    if  magic_str != actual_str {
        return Err(LevelParseError::InvalidMagicString(actual_str.to_string()));
    }

    Ok(())
}

fn parse_level_header(compressed_data: &Vec<u8>, offset: usize, decompressed_byte_count: usize) -> Result<LevelHeader, LevelParseError> {
    let compressed_data_without_offset = compressed_data[offset..compressed_data.len()].to_vec();
    let data_u16 = rlew::decode(&compressed_data_without_offset, RLEW_TAG, Some(decompressed_byte_count)).map_err(|e| {
        LevelParseError::LevelHeaderRlewDecodeError { error: e }
    })?;
    let mut data = vec![0; 2 * data_u16.len()];
    LittleEndian::write_u16_into(&data_u16, &mut data);

    let planes_num = 3;
    let mut plane_headers = Vec::with_capacity(planes_num);
    for i in 0..planes_num {
        let plane_header = read_plane_header(&data, planes_num, i);
        plane_headers.push(plane_header);
    }

    let width_offset = planes_num * 6;
    if width_offset >= data.len() {
        return Err(LevelParseError::UnexpectedEndOfData);
    }
    let width = LittleEndian::read_u16(
        &data[width_offset..(width_offset + 2)]
    );

    let height_offset = width_offset + 2;
    if height_offset >= data.len() {
        return Err(LevelParseError::UnexpectedEndOfData);
    }
    let height = LittleEndian::read_u16(
        &data[height_offset..(height_offset + 2)]
    );

    let name = parse_name(&data, height_offset + 2)?;

    Ok(LevelHeader {
        plane_headers: plane_headers,
        name: name,
        width: width,
        height: height
    })
}

struct LevelHeader {
    plane_headers: Vec<PlaneHeader>,
    name: String,
    width: u16,
    height: u16
}

fn read_plane_header(data: &Vec<u8>, planes_num: usize, plane_num: usize) -> PlaneHeader {
    let plane_offset_offset = plane_num * 4;
    let plane_length_offset = planes_num * 4 + plane_num * 2;
    let plane_offset = LittleEndian::read_u32(&data[plane_offset_offset..(plane_offset_offset + 4)]);
    let plane_compressed_length = LittleEndian::read_u16(&data[plane_length_offset..(plane_length_offset + 2)]);
    PlaneHeader {
        offset: plane_offset,
        compressed_length: plane_compressed_length
    }
}

fn parse_name(data: &Vec<u8>, offset: usize) -> Result<String, LevelParseError> {
    let null_index = data[offset..(offset + 16)]
        .iter()
        .position(|&b| b == 0)
        .ok_or(LevelParseError::InvalidName)?;
    let name_bytes_with_null = &data[offset..(offset + null_index + 1)];
    Ok(CStr::from_bytes_with_nul(name_bytes_with_null).map_err(|_|
        LevelParseError::InvalidName
    )?.to_str().map_err(|_| LevelParseError::InvalidName)?.to_string())
}

struct PlaneHeader {
    offset: u32,
    compressed_length: u16
}

#[derive(Debug, PartialEq)]
pub struct Level {
    pub name: String,
    pub width: u16,
    pub height: u16,
    pub planes: Vec<Plane>
}

#[derive(Debug, PartialEq)]
pub struct Plane {
    pub data: Vec<u16>
}

#[derive(Debug, PartialEq)]
pub enum LevelParseError {
    UnexpectedEndOfData,
    InvalidMagicString(String),
    InvalidPlaneLength { plane: usize, length: usize },
    CarmackDecompressionError { plane: usize, error: carmack::DecompressionError },
    LevelHeaderRlewDecodeError { error: rlew::RlewDecodeError },
    PlaneRlewDecodeError { plane: usize, error: rlew::RlewDecodeError },
    InvalidName
}

impl fmt::Display for LevelParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LevelParseError::UnexpectedEndOfData =>
                write!(f, "Unexpected end of data"),
            LevelParseError::InvalidMagicString(ref s) =>
                write!(f, "Invalid magic string: {}", s),
            LevelParseError::InvalidPlaneLength { plane, length } =>
                write!(f, "Invalid plane length for plane {}: {}", plane, length),
            LevelParseError::CarmackDecompressionError { plane, ref error } =>
                write!(f, "Carmack decompression error for plane {}: {}", plane, error),
            LevelParseError::LevelHeaderRlewDecodeError { ref error } =>
                write!(f, "RLEW decode error for the level header: {}", error),
            LevelParseError::PlaneRlewDecodeError { plane, ref error } =>
                write!(f, "RLEW decode error for plane {}: {}", plane, error),
            LevelParseError::InvalidName =>
                write!(f, "Invalid level name")
        }?;
        Ok(())
    }
}