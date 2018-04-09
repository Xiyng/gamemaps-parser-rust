use super::*;

#[test]
fn decodes_a_single_repeated_value() {
    let value = 0xfe;
    let count = 2;

    let test_data_u16 = vec![0xfe, 2];
    let mut test_data = vec![0; 2 * test_data_u16.len()];
    LittleEndian::write_u16_into(&test_data_u16, &mut test_data);

    assert_eq!(
        decode(&test_data),
        Ok(vec![value; count as usize])
    );
}

#[test]
fn decodes_two_repeated_values() {
    let test_data_u16 = vec![0xfe, 2, 0xab, 3];
    let mut test_data = vec![0; 2 * test_data_u16.len()];
    LittleEndian::write_u16_into(&test_data_u16, &mut test_data);

    assert_eq!(
        decode(&test_data),
        Ok(vec![0xfe, 0xfe, 0xab, 0xab, 0xab])
    );
}