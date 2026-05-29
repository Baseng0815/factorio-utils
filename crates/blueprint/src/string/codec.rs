use std::io::{Read, Write};

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;

use crate::error::{Error, Result};

pub fn base64_decode(s: &str) -> Result<Vec<u8>> {
    Ok(B64.decode(s.as_bytes())?)
}

pub fn base64_encode(bytes: &[u8]) -> String {
    B64.encode(bytes)
}

pub fn zlib_inflate(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(bytes);
    let mut out = Vec::with_capacity(bytes.len() * 4);
    decoder.read_to_end(&mut out).map_err(|e| Error::Zlib(e.to_string()))?;
    Ok(out)
}

pub fn zlib_deflate(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::with_capacity(bytes.len()), Compression::default());
    encoder.write_all(bytes).map_err(|e| Error::Zlib(e.to_string()))?;
    encoder.finish().map_err(|e| Error::Zlib(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_zlib() {
        let payload = b"hello blueprint world";
        let compressed = zlib_deflate(payload).unwrap();
        let back = zlib_inflate(&compressed).unwrap();
        assert_eq!(back, payload);
    }

    #[test]
    fn roundtrip_base64() {
        let payload = vec![0u8, 1, 2, 3, 255, 254];
        let encoded = base64_encode(&payload);
        let back = base64_decode(&encoded).unwrap();
        assert_eq!(back, payload);
    }
}
