use super::*;

#[test]
fn does_not_parse_empty_data() {
    assert_eq!(
        parse(&Vec::new()),
        Err(HeaderParseError::UnexpectedEndOfFile)
    );
}