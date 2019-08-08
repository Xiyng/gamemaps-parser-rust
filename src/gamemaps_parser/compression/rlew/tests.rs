use super::*;

extern crate byteorder;

use self::byteorder::{LittleEndian, WriteBytesExt};

const RLEW_TAG: u16 = 0xfefe;
const WOLF3D_RLEW_TAG: u16 = 0xabcd;

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
    let actual = decode(&compressed_u8, RLEW_TAG, None);
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

#[test]
fn decodes_wolf1_map1_plane2() {
    let compressed_data = vec![0x00, 0x20, 0xcd, 0xab, 0x00, 0x10, 0x00, 0x00];
    let decompressed_data = decode(&compressed_data, WOLF3D_RLEW_TAG, None).unwrap();
    assert_ne!(decompressed_data.len(), 0); // This is not really needed but it can clean up the failure message.
    assert_eq!(decompressed_data, vec![0x0000; 0x1000])
}