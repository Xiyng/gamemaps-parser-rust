extern crate byteorder;

use self::byteorder::*;

pub fn parse(data: &Vec<u8>) -> HeaderData {
    let mut header = HeaderData {
        rlew: 0,
        level_offsets: [0; 100],
        tile_info: Vec::new()
    };

    let rlew_tag_start = 0;
    let rlew_tag_end = 2;
    header.rlew = LittleEndian::read_u16(&data[rlew_tag_start..rlew_tag_end]);

    let level_offsets_start = rlew_tag_end;
    let level_offsets_end = level_offsets_start + header.level_offsets.len() * 4;
    LittleEndian::read_u32_into(
        &data[level_offsets_start..level_offsets_end],
        &mut header.level_offsets
    );

    let tile_info_start = level_offsets_end;
    header.tile_info = data[tile_info_start..].to_vec();

    header
}

pub struct HeaderData {
    pub rlew: u16,
    pub level_offsets: [u32; 100],
    pub tile_info: Vec<u8>
}