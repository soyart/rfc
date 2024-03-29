use base64::{self, engine::general_purpose::STANDARD as b64_engine};

use std::io::{Read, Write};

use crate::rfc::error::RfcError;

pub fn encode_b64_buf<R>(src: &mut R, src_len: usize) -> Result<Vec<u8>, RfcError>
where
    R: Read,
{
    let len_in_b64 = prealloc_to_b64(src_len);
    let mut buf = Vec::with_capacity(len_in_b64);
    encode_b64(src, &mut buf)?;

    Ok(buf)
}

pub fn decode_b64_buf<R>(src: &mut R, encoded_len: usize) -> Result<Vec<u8>, RfcError>
where
    R: Read,
{
    let len_in_plain = prealloc_from_b64(encoded_len);
    let mut buf = Vec::with_capacity(len_in_plain);
    decode_b64(src, &mut buf)?;

    Ok(buf)
}

pub fn encode_b64<S, D>(src: &mut S, dst: &mut D) -> Result<usize, RfcError>
where
    S: Read,
    D: Write,
{
    let mut encoder = base64::write::EncoderWriter::new(dst, &b64_engine);

    let written = std::io::copy(src, &mut encoder).map_err(|err| RfcError::IoError(err))?;

    Ok(written as usize)
}

pub fn decode_b64<S, D>(src: &mut S, dst: &mut D) -> Result<(), RfcError>
where
    S: Read,
    D: Write,
{
    let mut decoder = base64::read::DecoderReader::new(src, &b64_engine);

    std::io::copy(&mut decoder, dst).map_err(|err| RfcError::IoError(err))?;

    Ok(())
}

pub fn prealloc_to_b64(len: usize) -> usize {
    (len * 4) / 3 + 4
}

pub fn prealloc_from_b64(b64_len: usize) -> usize {
    (b64_len * 3 / 4) + 4
}

#[test]
fn test_b64() {
    let filename = "./Cargo.toml";
    let mut infile = std::fs::File::open(filename).unwrap();
    let infile_len = infile.metadata().unwrap().len() as usize;
    let mut encoded = Vec::with_capacity(prealloc_to_b64(infile_len));
    let mut decoded = Vec::with_capacity(infile_len);

    encode_b64(&mut infile, &mut encoded).unwrap();
    decode_b64(&mut encoded.as_slice(), &mut decoded).unwrap();

    let original_bytes = std::fs::read(filename).unwrap();

    assert_eq!(original_bytes, decoded);
}
