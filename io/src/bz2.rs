use std::io::Read;

use bzip2::read::{BzDecoder, BzEncoder};

/// Decompresses a Bzip2 compressed `Vec<u8>` into a decompressed `Vec<u8>`.
///
/// This function takes a vector of bytes representing the compressed data and decompresses it
/// into a `Vec<u8>`. It also takes the expected length of the decompressed data. The decompressed
/// data is returned as a new `Vec<u8>`.
///
/// # Arguments
///
/// * `bytes` - A vector containing the compressed Bzip2 data.
/// * `decompress_length` - The expected length of the decompressed data, in bytes.
///
/// # Return
///
/// Returns a `Vec<u8>` containing the decompressed data.
///
/// # Example
///
/// ```rust
/// use io::{bz2_compress, bz2_decompress};
///
/// let compressed: Vec<u8> = bz2_compress("Hello world!".as_bytes().to_vec());
/// let decompressed: Vec<u8> = bz2_decompress(compressed, 12, false, 0);
/// assert_eq!("Hello world!", String::from_utf8(decompressed).unwrap());
/// ```
///
/// # Panics
///
/// This function will panic if the decompressed data's size does not match the expected length,
/// as indicated by `decompress_length`. Ensure the correct value is provided for `decompress_length`
/// to avoid this panic.
///
/// # Safety
///
/// This function does not use `unsafe` code, but callers must ensure that the provided
/// `decompress_length` accurately reflects the actual size of the decompressed data. If the length
/// is incorrect, the function will panic with an error message.
pub fn bz2_decompress(
    bytes: Vec<u8>,
    decompress_length: usize,
    prepend_header: bool,
    offset: usize,
) -> Vec<u8> {
    let compressed: Vec<u8> = if prepend_header {
        let mut header = Vec::with_capacity(4 + bytes.len() - offset);
        header.extend_from_slice(b"BZh1");
        header.extend_from_slice(&bytes[offset..]);
        header
    } else {
        bytes
    };
    let mut decompressed: Vec<u8> = vec![0u8; decompress_length];
    BzDecoder::new(&compressed[..])
        .read_exact(&mut decompressed)
        .expect("Bzip2 could not decompress!");
    if decompress_length != decompressed.len() {
        panic!("Decompressed data size does not match expected length.");
    }
    return decompressed;
}

/// Compresses a `Vec<u8>` into a Bzip2 compressed `Vec<u8>`.
///
/// This function takes a vector of bytes and compresses it using the Bzip2 compression algorithm,
/// returning a new vector containing the compressed data.
///
/// # Arguments
///
/// * `bytes` - A vector containing the data to be compressed.
///
/// # Return
///
/// Returns a `Vec<u8>` containing the compressed data.
///
/// # Example
///
/// ```rust
/// use io::{bz2_compress, bz2_decompress};
///
/// let compressed: Vec<u8> = bz2_compress("Hello world!".as_bytes().to_vec());
/// let decompressed: Vec<u8> = bz2_decompress(compressed, 12, false, 0);
/// assert_eq!(String::from_utf8(decompressed).unwrap(), "Hello world!");
/// ```
///
/// # Panics
///
/// This function will panic if the compression process fails for any reason. Ensure that the input
/// data is valid and that the compression process can complete successfully.
///
/// # Safety
///
/// This function does not use `unsafe` code, but it is important to ensure that the input data
/// is valid and does not contain any invalid bytes that could cause issues during the compression.
/// The function uses `read_to_end()` to fill the `compressed` vector with the result.
pub fn bz2_compress(bytes: Vec<u8>) -> Vec<u8> {
    let mut compressed: Vec<u8> = Vec::new();
    BzEncoder::new(&bytes[..], bzip2::Compression::best())
        .read_to_end(&mut compressed)
        .expect("Bzip2 could not compress!");
    return compressed;
}
