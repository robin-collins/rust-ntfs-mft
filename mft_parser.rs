use crate::utils::{read_bytes, read_u16, read_u32, read_u64, read_string};
use anyhow::{Result, Context};
use std::collections::HashMap;

// Define constants for MFT Entry header offsets and sizes
const FILE_SIGNATURE_OFFSET: usize = 0;
const FILE_SIGNATURE_SIZE: usize = 4;
const FILE_RECORD_NUMBER_OFFSET: usize = 44;
const FILE_RECORD_NUMBER_SIZE: usize = 6;

// Define a struct to hold the parsed MFT entry data
#[derive(Debug)]
pub struct MftEntry {
    pub signature: String,
    pub record_number: u64,
    // Add more fields as necessary to represent the MFT entry data
}

impl MftEntry {
    pub fn parse(entry_data: &[u8]) -> Result<Self> {
        // Parse the signature
        let signature = read_string(entry_data, FILE_SIGNATURE_OFFSET, FILE_SIGNATURE_SIZE)
            .with_context(|| "Failed to parse file signature")?;

        // Parse the record number
        let record_number = read_u64(entry_data, FILE_RECORD_NUMBER_OFFSET)
            .with_context(|| "Failed to parse file record number")?;

        // Create and return the MftEntry struct
        Ok(MftEntry {
            signature,
            record_number,
            // Initialize other fields as necessary
        })
    }
}

// Define a struct to represent the MFT parser
pub struct MftParser;

impl MftParser {
    pub fn new() -> Self {
        MftParser
    }

    pub fn parse_mft_entries(&self, mft_data: Vec<u8>) -> Result<Vec<MftEntry>> {
        let mut entries = Vec::new();
        let mut offset = 0;

        while offset < mft_data.len() {
            // Assuming the default size of an MFT entry is 1024 bytes
            let mft_entry_size = 1024;

            // Check if we have enough data left to read a full MFT entry
            if offset + mft_entry_size > mft_data.len() {
                break;
            }

            // Extract the MFT entry data
            let entry_data = &mft_data[offset..offset + mft_entry_size];

            // Parse the MFT entry
            let entry = MftEntry::parse(entry_data)
                .with_context(|| format!("Failed to parse MFT entry at offset {}", offset))?;

            // Add the parsed entry to the list
            entries.push(entry);

            // Move to the next MFT entry
            offset += mft_entry_size;
        }

        Ok(entries)
    }
}

// Add more methods and logic as needed for your project.

// Utility functions to read data from a byte slice
fn read_string(entry_data: &[u8], offset: usize, size: usize) -> Result<String> {
    let end = offset + size;
    let bytes = &entry_data[offset..end];
    let string = String::from_utf8_lossy(bytes).to_string();
    Ok(string)
}

fn read_u64(entry_data: &[u8], offset: usize) -> Result<u64> {
    let end = offset + FILE_RECORD_NUMBER_SIZE;
    let bytes = &entry_data[offset..end];
    let mut cursor = std::io::Cursor::new(bytes);
    let value = cursor.read_u64::<byteorder::LittleEndian>()
        .with_context(|| format!("Failed to read u64 from entry data at offset {}", offset))?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mft_entry_parsing() {
        // Create a fake MFT entry for testing purposes
        let mut fake_mft_entry = vec![0; 1024];
        // Set a fake signature "FILE"
        fake_mft_entry[FILE_SIGNATURE_OFFSET..FILE_SIGNATURE_OFFSET + FILE_SIGNATURE_SIZE].copy_from_slice(b"FILE");
        // Set a fake record number
        fake_mft_entry[FILE_RECORD_NUMBER_OFFSET..FILE_RECORD_NUMBER_OFFSET + FILE_RECORD_NUMBER_SIZE].copy_from_slice(&12345u64.to_le_bytes()[..FILE_RECORD_NUMBER_SIZE]);

        // Parse the fake MFT entry
        let entry = MftEntry::parse(&fake_mft_entry).unwrap();

        // Check the parsed values
        assert_eq!(entry.signature, "FILE");
        assert_eq!(entry.record_number, 12345);
    }
}
