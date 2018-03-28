extern crate byteorder;

use std::panic;
use self::byteorder::*;

pub fn parse(data: &Vec<u8>) -> Result<HeaderData, HeaderParseError> {
    let mut header = HeaderData {
        level_offsets: [0; 100],
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

    let level_offsets_start = rlew_tag_end;
    let level_offsets_end = level_offsets_start + header.level_offsets.len() * 4;
    header.level_offsets = panic::catch_unwind(|| {
        let mut level_offsets = header.level_offsets;
        LittleEndian::read_u32_into(
            &data[level_offsets_start..level_offsets_end],
            &mut level_offsets
        );
        level_offsets
    }).map_err(|_| HeaderParseError::UnexpectedEndOfFile)?;

    let tile_info_start = level_offsets_end;
    header.tile_info = panic::catch_unwind(||
        data[tile_info_start..].to_vec()
    ).map_err(|_| HeaderParseError::UnexpectedEndOfFile)?;

    Ok(header)
}

pub struct HeaderData {
    pub level_offsets: [u32; 100],
    pub tile_info: Vec<u8>
}

pub enum HeaderParseError {
    UnexpectedEndOfFile,
    InvalidRlewTag(u16)
}