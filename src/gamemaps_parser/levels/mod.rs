#[cfg(test)]
mod tests;

extern crate byteorder;

use std::ffi::CStr;
use self::byteorder::*;

pub fn parse(data: &Vec<u8>, offset: u32) -> Result<Level, LevelParseError> {
    let offset_usize = offset as usize;

    let planes_num = 3;
    let mut planes = Vec::with_capacity(planes_num);
    for i in 0..planes_num {
        let plane_offset_offset = offset_usize + i * 4;
        let plane_offset = LittleEndian::read_u32(
            &data[plane_offset_offset..(plane_offset_offset + 4)]
        ) as usize;

        let plane_length_offset = offset_usize + planes_num * 4 + i * 2;
        let plane_length_raw = LittleEndian::read_u16(
            &data[plane_length_offset..(plane_length_offset + 2)]
        ) as usize;

        // TODO: Verify that plane_length_raw is even.
        let mut raw_plane_data = vec!(0; plane_length_raw / 2);
        LittleEndian::read_u16_into(
            &data[plane_offset..(plane_offset + plane_length_raw)],
            &mut raw_plane_data
        );

        // We'll skip uncompressing plane data for now and hope it's
        // uncompressed.
        planes.push(Plane { data: raw_plane_data });
    }

    let width_offset = offset_usize + planes_num * 6;
    let width = LittleEndian::read_u16(
        &data[width_offset..(width_offset + 2)]
    );

    let height_offset = width_offset + 2;
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
    InvalidName
}