use super::*;

#[test]
fn does_not_parse_empty_data() {
    assert_eq!(
        parse(&Vec::new()),
        Err(HeaderParseError::UnexpectedEndOfFile)
    );
}

#[test]
fn does_not_parse_invalid_rlew_tag() {
    assert_eq!(
        parse(&vec![0xfe, 0xef]),
        Err(HeaderParseError::InvalidRlewTag(0xeffe))
    );
}