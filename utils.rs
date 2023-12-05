use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use byteorder::{ReadBytesExt, LittleEndian};
use anyhow::{Result, Context};

pub fn read_bytes(file: &mut File, offset: u64, size: usize) -> Result<Vec<u8>> {
    let mut buffer = vec![0; size];
    file.seek(SeekFrom::Start(offset))
        .with_context(|| format!("Failed to seek to offset {}", offset))?;
    file.read_exact(&mut buffer)
        .with_context(|| format!("Failed to read {} bytes from offset {}", size, offset))?;
    Ok(buffer)
}

pub fn read_u16(file: &mut File, offset: u64) -> Result<u16> {
    file.seek(SeekFrom::Start(offset))
        .with_context(|| format!("Failed to seek to offset {}", offset))?;
    let value = file.read_u16::<LittleEndian>()
        .with_context(|| format!("Failed to read u16 from offset {}", offset))?;
    Ok(value)
}

pub fn read_u32(file: &mut File, offset: u64) -> Result<u32> {
    file.seek(SeekFrom::Start(offset))
        .with_context(|| format!("Failed to seek to offset {}", offset))?;
    let value = file.read_u32::<LittleEndian>()
        .with_context(|| format!("Failed to read u32 from offset {}", offset))?;
    Ok(value)
}

pub fn read_u64(file: &mut File, offset: u64) -> Result<u64> {
    file.seek(SeekFrom::Start(offset))
        .with_context(|| format!("Failed to seek to offset {}", offset))?;
    let value = file.read_u64::<LittleEndian>()
        .with_context(|| format!("Failed to read u64 from offset {}", offset))?;
    Ok(value)
}

pub fn read_string(file: &mut File, offset: u64, length: usize) -> Result<String> {
    let bytes = read_bytes(file, offset, length)?;
    let string = String::from_utf8_lossy(&bytes).to_string();
    Ok(string)
}

// Add more utility functions as needed for your project.
