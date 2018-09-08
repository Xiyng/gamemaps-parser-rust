use super::*;

extern crate byteorder;

use self::byteorder::{LittleEndian, WriteBytesExt};

const RLEW_TAG: u16 = 0xfefe;

struct SuccessTestData {
    compressed: Vec<u16>,
    decompressed: Vec<u16>
}

fn assert_success(test_data: SuccessTestData) {
    let mut compressed_u8 = Vec::with_capacity(2 * test_data.compressed.len());
    for x in test_data.compressed {
        compressed_u8.write_u16::<LittleEndian>(x);
    }
    let actual = decode(&compressed_u8, RLEW_TAG);
    let expected = Ok(test_data.decompressed);
    assert_eq!(actual, expected)
}

#[test]
fn decodes_a_single_repeated_value() {
    assert_success(SuccessTestData {
        compressed: vec![RLEW_TAG, 0xabcd, 2],
        decompressed: vec![0xabcd, 0xabcd]
    })
}

#[test]
fn decodes_two_repeated_values() {
    assert_success(SuccessTestData {
        compressed: vec![RLEW_TAG, 0xabcd, 2, RLEW_TAG, 0xbcde, 3],
        decompressed: vec![0xabcd, 0xabcd, 0xbcde, 0xbcde, 0xbcde]
    })
}