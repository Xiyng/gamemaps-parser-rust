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
        match compressed_u8.write_u16::<LittleEndian>(x) {
            Err(_) => assert!(false),
            _ => {}
        }
    }
    let actual = decode(&compressed_u8, RLEW_TAG);
    let expected = Ok(test_data.decompressed);
    assert_eq!(actual, expected)
}

#[test]
fn decodes_a_single_repeated_value() {
    assert_success(SuccessTestData {
        compressed: vec![0x04, 0x00, 0x00, 0x00, RLEW_TAG, 2, 0xabcd],
        decompressed: vec![0xabcd, 0xabcd]
    })
}

#[test]
fn decodes_two_repeated_values() {
    assert_success(SuccessTestData {
        compressed: vec![0x0a, 0x00, 0x00, 0x00, RLEW_TAG, 2, 0xabcd, RLEW_TAG, 3, 0xbcde],
        decompressed: vec![0xabcd, 0xabcd, 0xbcde, 0xbcde, 0xbcde]
    })
}