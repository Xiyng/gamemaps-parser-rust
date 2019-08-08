extern crate byteorder;

use self::byteorder::*;

use gamemaps_parser::compression::{carmack, rlew};

#[test]
fn decompresses_wolf1_map1_plane2() {
    let carmack_compressed_data = vec![0x08, 0x00, 0x00, 0x20, 0xcd, 0xab, 0x00, 0x10, 0x00, 0x00];
    let rlew_compressed_words = carmack::decompress(&carmack_compressed_data, 0).expect("Carmack decompression failed.");
    assert_eq!(4, rlew_compressed_words.len());
    let mut rlew_compressed_bytes = vec![0; 2 * rlew_compressed_words.len()];
        LittleEndian::write_u16_into(
            &rlew_compressed_words,
            &mut rlew_compressed_bytes
        );
    let decompressed_words = rlew::decode(&rlew_compressed_bytes, 0xabcd, None).expect("RLEW decoding failed.");
    assert_eq!(decompressed_words.len(), 4096)
}