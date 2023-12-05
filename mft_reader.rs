use crate::config::Config;
use crate::utils::{read_bytes, read_u16, read_u32, read_u64};
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{Seek, SeekFrom};

pub struct MftReader {
    file: File,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    mft_start_lcn: u64,
}

impl MftReader {
    pub fn new(config: &Config) -> Result<Self> {
        let mut file = File::open(&config.mft_file_path)
            .with_context(|| format!("Failed to open MFT file at {:?}", config.mft_file_path))?;

        // Read necessary boot sector values to calculate the MFT offset
        let bytes_per_sector = read_u16(&mut file, 11)?;
        let sectors_per_cluster = read_bytes(&mut file, 13, 1)?[0];
        let mft_start_lcn = read_u64(&mut file, 48)?;

        Ok(MftReader {
            file,
            bytes_per_sector,
            sectors_per_cluster,
            mft_start_lcn,
        })
    }

    pub fn read_mft_entry(&mut self, entry_index: u64) -> Result<Vec<u8>> {
        // Calculate the offset of the MFT entry
        let mft_offset = self.calculate_mft_offset(entry_index);

        // Seek to the MFT entry offset
        self.file.seek(SeekFrom::Start(mft_offset))
            .with_context(|| format!("Failed to seek to MFT entry at offset {}", mft_offset))?;

        // Read the MFT entry - assuming the default size of an MFT entry is 1024 bytes
        // This size can vary and should ideally be retrieved from the boot sector or $MFT record
        let mft_entry_size = 1024;
        let mft_entry = read_bytes(&mut self.file, mft_offset, mft_entry_size)?;

        Ok(mft_entry)
    }

    fn calculate_mft_offset(&self, entry_index: u64) -> u64 {
        // Calculate the byte offset of the MFT entry
        // This assumes the default size of an MFT entry is 1024 bytes
        let mft_entry_size = 1024;
        let cluster_size = u64::from(self.bytes_per_sector) * u64::from(self.sectors_per_cluster);
        let mft_offset = self.mft_start_lcn * cluster_size + entry_index * mft_entry_size;

        mft_offset
    }
}

// Add more methods as needed for your project.

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_mft_reader_initialization() {
        let config = Config {
            database_url: String::from("sqlite:mft_data.db"),
            mft_file_path: PathBuf::from("C:\\path\\to\\MFT"),
        };

        let mft_reader = MftReader::new(&config);
        assert!(mft_reader.is_ok());
    }

    #[test]
    fn test_mft_entry_reading() {
        let config = Config {
            database_url: String::from("sqlite:mft_data.db"),
            mft_file_path: PathBuf::from("C:\\path\\to\\MFT"),
        };

        let mut mft_reader = MftReader::new(&config).unwrap();
        let mft_entry = mft_reader.read_mft_entry(0);
        assert!(mft_entry.is_ok());
        assert_eq!(mft_entry.unwrap().len(), 1024);
    }
}
