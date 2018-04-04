extern crate byteorder;

use self::byteorder::*;
use super::*;

#[test]
fn parses_valid_data() {
    let test_data_offset = 10;
    let test_data = create_empty_test_data(test_data_offset);
    
    assert_eq!(
        parse(&test_data, test_data_offset),
        Ok(Level {
            name: "test".to_string(),
            width: 64,
            height: 64,
            planes: vec![
                Plane { data: Vec::new() },
                Plane { data: Vec::new() },
                Plane { data: Vec::new() }
            ]
        })
    );
}

fn create_empty_test_data(offset: u32) -> Vec<u8> {
    let mut test_data = Vec::new();

    let magic_str = "TED5v1.0";
    test_data.append(&mut magic_str.bytes().collect());

    test_data.append(&mut vec![0; offset as usize - magic_str.len()]); // offset

    test_data.write_u32::<LittleEndian>(0).unwrap(); // plane 1 offset
    test_data.write_u32::<LittleEndian>(0).unwrap(); // plane 2 offset
    test_data.write_u32::<LittleEndian>(0).unwrap(); // plane 3 offset

    test_data.write_u16::<LittleEndian>(0).unwrap(); // plane 1 length
    test_data.write_u16::<LittleEndian>(0).unwrap(); // plane 2 length
    test_data.write_u16::<LittleEndian>(0).unwrap(); // plane 3 length

    test_data.write_u16::<LittleEndian>(64).unwrap(); // width
    test_data.write_u16::<LittleEndian>(64).unwrap(); // height

    let name = "test";
    test_data.append(&mut name.bytes().collect()); // name
    test_data.append(&mut vec![0; 16 - name.len()]); // null characters for name

    test_data
}