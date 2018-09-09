use super::*;

extern crate byteorder;

struct SuccessTestData {
    compressed: Vec<u8>,
    decompressed: Vec<u16>
}

fn assert_success(test_data: SuccessTestData) {
    let actual = decompress(&test_data.compressed);
    let expected = Ok(test_data.decompressed);
    assert_eq!(actual, expected)
}

#[test]
fn works_with_empty_data() {
    assert_success(SuccessTestData {
        compressed: vec![],
        decompressed: vec![]
    })
}

#[test]
fn does_not_modify_uncompressed_data() {
    assert_success(SuccessTestData {
        compressed: vec![0xcd, 0x00],
        decompressed: vec![0x00cd]
    })
}

#[test]
fn decompresses_with_one_near_pointer() {
    assert_success(SuccessTestData {
        compressed: vec![0xcd, 0x00, 0x01, 0xa7, 0x02],
        decompressed: vec![0x00cd, 0x00cd]
    })
}

#[test]
fn decompresses_with_one_far_pointer() {
    assert_success(SuccessTestData {
        compressed: vec![0xcd, 0x00, 0x01, 0xa8, 0x00, 0x00],
        decompressed: vec![0x00cd, 0x00cd]
    })
}

#[test]
fn decompresses_data_with_one_near_and_one_far_pointer() {
    assert_success(SuccessTestData {
        compressed: vec![0xcd, 0x00, 0x01, 0xa7, 0x02, 0xde, 0x00, 0x01, 0xa8, 0x00, 0x00],
        decompressed: vec![0x00cd ,0x00cd, 0x00de, 0x00cd]
    })
}