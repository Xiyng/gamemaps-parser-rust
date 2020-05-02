use std::io::BufReader;

use super::*;

#[test]
fn does_not_parse_empty_data() {
    assert_eq!(
        parse_vec(&Vec::new()),
        Err(HeaderParseError::UnexpectedEndOfFile)
    );
}

#[test]
fn does_not_parse_invalid_rlew_tag() {
    assert_eq!(
        parse_vec(&vec![0xfe, 0xef]),
        Err(HeaderParseError::InvalidRlewTag(0xeffe))
    );
}

#[test]
fn does_not_parse_valid_rlew_tag_and_missing_level_offsets() {
    assert_eq!(
        parse_vec(&vec![0xcd, 0xab]),
        Err(HeaderParseError::UnexpectedEndOfFile)
    );
}

#[test]
fn does_not_parse_valid_rlew_tag_and_too_short_level_offsets() {
    assert_eq!(
        parse_vec(&vec![0xcd, 0xab, 0x00]),
        Err(HeaderParseError::UnexpectedEndOfFile)
    );
}

#[test]
fn parses_valid_file_with_zero_offsets() {
    let mut rlew_tag = vec![0xcd, 0xab];
    let mut level_offsets = vec![0; 400];

    let mut test_data = Vec::with_capacity(rlew_tag.len() + level_offsets.len());
    test_data.append(&mut rlew_tag);
    test_data.append(&mut level_offsets);

    assert_eq!(parse_vec(&test_data), Ok(HeaderData {
        level_offsets: Vec::new(),
        tile_info: Vec::new()
    }));
}

#[test]
fn parses_valid_file_with_non_zero_offsets() {
    let mut rlew_tag = vec![0xcd, 0xab];
    let mut level_offsets = Vec::with_capacity(400);
    level_offsets.append(&mut vec![0xfe, 0x00, 0x00, 0x00]);
    level_offsets.append(&mut vec![0; 392]);
    level_offsets.append(&mut vec![0xef, 0x00, 0x00, 0x00]);

    let mut level_offsets_u32_buf = [0; 100];
    LittleEndian::read_u32_into(&level_offsets, &mut level_offsets_u32_buf);
    let level_offsets_u32 = level_offsets_u32_buf.to_vec();

    let mut test_data = Vec::with_capacity(rlew_tag.len() + level_offsets.len());
    test_data.append(&mut rlew_tag);
    test_data.append(&mut level_offsets);

    assert_eq!(parse_vec(&test_data), Ok(HeaderData {
        level_offsets: level_offsets_u32,
        tile_info: Vec::new()
    }));
}

fn parse_vec(vec: &Vec<u8>) -> Result<HeaderData, HeaderParseError> {
    let reader: Box<_> = BufReader::new(&vec[..]).into();
    parse(reader)
}