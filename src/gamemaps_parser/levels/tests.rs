extern crate byteorder;

use self::byteorder::*;
use super::*;

#[test]
fn parses_valid_data_with_zero_offset() {
    let test_data_offset = 0;
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

#[test]
fn parses_valid_data_with_non_zero_offset() {
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

    test_data.append(&mut vec![0; offset as usize]); // offset

    test_data.write_u32::<LittleEndian>(0).unwrap(); // plane 1 offset
    test_data.write_u32::<LittleEndian>(0).unwrap(); // plane 2 offset
    test_data.write_u32::<LittleEndian>(0).unwrap(); // plane 3 offset

    test_data.write_u16::<LittleEndian>(0).unwrap(); // plane 1 length
    test_data.write_u16::<LittleEndian>(0).unwrap(); // plane 2 length
    test_data.write_u16::<LittleEndian>(0).unwrap(); // plane 3 length

    test_data.write_u16::<LittleEndian>(64).unwrap(); // width
    test_data.write_u16::<LittleEndian>(64).unwrap(); // height

    test_data.append(&mut vec![0x74, 0x65, 0x73, 0x74]); // name = test
    test_data.append(&mut vec![0; 12]); // null characters for name

    test_data
}