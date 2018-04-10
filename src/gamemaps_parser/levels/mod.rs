#[cfg(test)]
mod tests;

extern crate byteorder;

use std::ffi::CStr;
use std::fmt;
use gamemaps_parser::rlew;
use self::byteorder::*;

pub fn parse(data: &Vec<u8>, offset: u32) -> Result<Level, LevelParseError> {
    validate_magic_str(&data)?;

    let offset_usize = offset as usize;

    let planes_num = 3;
    let mut planes = Vec::with_capacity(planes_num);
    for i in 0..planes_num {
        let raw_plane_data = read_raw_plane_data(
            data,
            offset_usize,
            planes_num,
            i
        )?;
        let rlew_decoded_data = rlew::decode(&raw_plane_data).map_err(|e|
            LevelParseError::RlewDecodeError {
                plane: i,
                error: e
            }
        )?;

        // Let's do only RLEW decoding for now and add other decompression
        // methods when we need them.
        planes.push(Plane { data: rlew_decoded_data });
    }

    let width_offset = offset_usize + planes_num * 6;
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

    Ok(Level {
        name: name,
        width: width,
        height: height,
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

fn read_raw_plane_data(data: &Vec<u8>, offset: usize, planes_num: usize, plane_num: usize) -> Result<Vec<u8>, LevelParseError> {
    let plane_offset_offset = offset + plane_num * 4;
    let plane_length_offset = offset + planes_num * 4 + plane_num * 2;

    if plane_length_offset >= data.len() {
        return Err(LevelParseError::UnexpectedEndOfData);
    }

    let plane_offset = LittleEndian::read_u32(
        &data[plane_offset_offset..(plane_offset_offset + 4)]
    ) as usize;

    let plane_length_raw = LittleEndian::read_u16(
        &data[plane_length_offset..(plane_length_offset + 2)]
    ) as usize;

    if plane_length_raw % 2 != 0 {
        return Err(LevelParseError::InvalidPlaneLength {
            plane: plane_num,
            length: plane_length_raw
        });
    }

    // let mut raw_plane_data = vec!(0; plane_length_raw / 2);
    // LittleEndian::read_u16_into(
    //     &data[plane_offset..(plane_offset + plane_length_raw)],
    //     &mut raw_plane_data
    // );

    // Ok(raw_plane_data)
    Ok(data[plane_offset..(plane_offset + plane_length_raw)].to_vec())
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
    RlewDecodeError { plane: usize, error: rlew::RlewDecodeError },
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
            LevelParseError::RlewDecodeError { plane, ref error } =>
                write!(f, "RLEW decode error for plane {}: {}", plane, error),
            LevelParseError::InvalidName =>
                write!(f, "Invalid level name")
        }?;
        Ok(())
    }
}