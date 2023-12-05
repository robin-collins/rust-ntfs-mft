use ntfs_mft_lib::config::Config;
use ntfs_mft_lib::mft_reader::MftReader;
use ntfs_mft_lib::mft_parser::MftEntry;
use ntfs_mft_lib::data_structurer::{StructuredData, DbEntry};
use ntfs_mft_lib::database_interface::DatabaseInterface;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    // Load the configuration
    let config = Config::new().context("Failed to load configuration")?;

    // Initialize the MFT reader
    let mut mft_reader = MftReader::new(&config).context("Failed to initialize MFT reader")?;

    // Initialize the database interface
    let database_interface = DatabaseInterface::new(&config).context("Failed to initialize database interface")?;

    // Read and parse the MFT entries
    let mut structured_data = StructuredData { entries: Vec::new() };
    let mut entry_index = 0;

    loop {
        // Read an MFT entry
        match mft_reader.read_mft_entry(entry_index) {
            Ok(entry_data) => {
                // Parse the MFT entry
                match MftEntry::parse(&entry_data) {
                    Ok(mft_entry) => {
                        // Convert the parsed MFT entry to a database entry
                        let db_entry = DbEntry::from_mft_entry(mft_entry);
                        structured_data.entries.push(db_entry);
                    },
                    Err(e) => eprintln!("Failed to parse MFT entry at index {}: {}", entry_index, e),
                }
            },
            Err(e) => {
                eprintln!("Failed to read MFT entry at index {}: {}", entry_index, e);
                break; // Stop reading if we encounter an error
            }
        }

        entry_index += 1; // Increment the entry index to read the next MFT entry
    }

    // Store the structured data in the database
    database_interface.store_data(&structured_data).context("Failed to store data in the database")?;

    println!("MFT data has been successfully read, parsed, and stored in the database.");

    Ok(())
}
9/