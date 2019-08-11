use super::*;

extern crate byteorder;

struct SuccessTestData {
    compressed: Vec<u8>,
    decompressed: Vec<u16>
}

fn assert_success(test_data: SuccessTestData, offset: usize) {
    let actual = decompress(&test_data.compressed, offset);
    let expected = Ok(test_data.decompressed);
    assert_eq!(actual, expected)
}

#[test]
fn does_not_modify_uncompressed_data() {
    assert_success(SuccessTestData {
        compressed: vec![0x01, 0x00, 0xcd, 0x00],
        decompressed: vec![0x00cd]
    }, 0)
}

#[test]
fn decompresses_with_one_near_pointer() {
    assert_success(SuccessTestData {
        compressed: vec![0x02, 0x00, 0xcd, 0x00, 0x01, 0xa7, 0x01],
        decompressed: vec![0x00cd, 0x00cd]
    }, 0)
}

#[test]
fn decompresses_with_one_far_pointer() {
    assert_success(SuccessTestData {
        compressed: vec![0x04, 0x00, 0xcd, 0x00, 0x01, 0xa8, 0x00, 0x00],
        decompressed: vec![0x00cd, 0x00cd]
    }, 0)
}

#[test]
fn decompresses_data_with_one_near_and_one_far_pointer() {
    assert_success(SuccessTestData {
        compressed: vec![0x08, 0x00, 0xcd, 0x00, 0x01, 0xa7, 0x02, 0xde, 0x00, 0x01, 0xa8, 0x01, 0x00],
        decompressed: vec![0x00cd, 0x00cd, 0x00de, 0x00cd]
    }, 0)
}

#[test]
fn decompresses_data_with_high_byte_0xa7() {
    assert_success(SuccessTestData {
        compressed: vec![0x02, 0x00, 0x00, 0xa7, 0xcd],
        decompressed: vec![0xcda7]
    }, 0)
}

#[test]
fn decompresses_data_with_high_byte_0xa8() {
    assert_success(SuccessTestData {
        compressed: vec![0x02, 0x00, 0x00, 0xa8, 0xcd],
        decompressed: vec![0xcda8]
    }, 0)
}