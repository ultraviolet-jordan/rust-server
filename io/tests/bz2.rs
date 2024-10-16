use io::{bz2_compress, bz2_decompress};

#[test]
fn test_decompress() {
    let compressed: Vec<u8> = bz2_compress("Hello world!".as_bytes().to_vec());
    let decompressed: Vec<u8> = bz2_decompress(compressed, 12, false, 0);
    assert_eq!("Hello world!", String::from_utf8(decompressed).unwrap());
}

#[test]
fn test_compress() {
    let compressed: Vec<u8> = bz2_compress("Hello world!".as_bytes().to_vec());
    let decompressed: Vec<u8> = bz2_decompress(compressed, 12, false, 0);
    assert_eq!(String::from_utf8(decompressed).unwrap(), "Hello world!");
}
