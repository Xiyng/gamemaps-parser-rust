#[cfg(test)]
mod tests;

extern crate byteorder;

use std::panic;
use self::byteorder::*;

pub fn parse(data: &Vec<u8>) -> Result<HeaderData, HeaderParseError> {
    let mut header = HeaderData {
        level_offsets: Vec::new(),
        tile_info: Vec::new()
    };

    let rlew_tag_start = 0;
    let rlew_tag_end = 2;
    panic::catch_unwind(||
        LittleEndian::read_u16(&data[rlew_tag_start..rlew_tag_end])
    ).map_err(|_|
        HeaderParseError::UnexpectedEndOfFile
    ).and_then(|rlew_tag| match rlew_tag {
        0xabcd => Ok(rlew_tag),
        _      => Err(HeaderParseError::InvalidRlewTag(rlew_tag))
    })?;

    let level_offsets_len = 100;
    let level_offsets_start = rlew_tag_end;
    let level_offsets_end = level_offsets_start + level_offsets_len * 4;
    header.level_offsets = panic::catch_unwind(|| {
        let mut level_offsets_buf = [0; 100];
        LittleEndian::read_u32_into(
            &data[level_offsets_start..level_offsets_end],
            &mut level_offsets_buf
        );

        let mut level_offsets = level_offsets_buf.to_vec();
        let last_existing_index = level_offsets.iter().rposition(|offset| *offset != 0);
        match last_existing_index {
            Some(index) => level_offsets.truncate(index + 1),
            None        => level_offsets.clear()
        };
        level_offsets
    }).map_err(|_| HeaderParseError::UnexpectedEndOfFile)?;

    let tile_info_start = level_offsets_end;
    header.tile_info = panic::catch_unwind(||
        data[tile_info_start..].to_vec()
    ).map_err(|_| HeaderParseError::UnexpectedEndOfFile)?;

    Ok(header)
}

#[derive(Debug, PartialEq)]
pub struct HeaderData {
    pub level_offsets: Vec<u32>,
    pub tile_info: Vec<u8>
}

#[derive(Debug, PartialEq)]
pub enum HeaderParseError {
    UnexpectedEndOfFile,
    InvalidRlewTag(u16)
}