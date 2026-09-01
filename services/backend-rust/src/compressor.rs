use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::error::Error;
use std::io::Write;

pub struct SovereignCompressor;

impl SovereignCompressor {
    /// Compresses a raw string payload into compressed byte streams using DEFLATE
    pub fn compress_payload(raw_data: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        // Initialize the encoder with an optimal compression level trade-off
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());

        // Write string slice data directly through compression channels
        encoder.write_all(raw_data.as_bytes())?;

        // Finalize writing stream processes to recover byte slices
        let compressed_bytes = encoder.finish()?;

        println!("🗜️ Data compression complete. Extracted compressed block bytes: {}", compressed_bytes.len());
        Ok(compressed_bytes)
    }

    /// Decompresses a compressed DEFLATE byte stream back into its original string layout
    pub fn decompress_payload(compressed_bytes: &[u8]) -> Result<String, Box<dyn Error>> {
        use flate2::write::DeflateDecoder;

        let mut decoder = DeflateDecoder::new(Vec::new());
        decoder.write_all(compressed_bytes)?;

        let decompressed_bytes = decoder.finish()?;
        let decompressed_string = String::from_utf8(decompressed_bytes)?;

        Ok(decompressed_string)
    }
}
