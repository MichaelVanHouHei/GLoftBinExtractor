use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::env;

fn validate_input(args: &[String]) -> Result<&str, String> {
    if args.len() != 2 {
        return Err(format!("Usage: {} <input_file_path>", args[0]));
    }
    let input_path = Path::new(&args[1]);
    if input_path.extension().and_then(|s| s.to_str()) != Some("bin") {
        return Err("Input file must have .bin extension".to_string());
    }
    Ok(&args[1])
}

fn read_and_validate_file(path: &str, delimiter: &[u8]) -> Result<Vec<u8>, String> {
    let mut file = File::open(path)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    if buffer.len() < delimiter.len() || buffer[0..delimiter.len()] != *delimiter {
        return Err("File does not start with valid delimiter".to_string());
    }
    Ok(buffer)
}

fn split_chunks<'a>(buffer: &'a [u8], delimiter: &'a [u8]) -> Vec<&'a [u8]> {
    let mut chunks = Vec::new();
    let mut start = 0;

    for i in 0..buffer.len() - delimiter.len() + 1 {
        if buffer[i..i + delimiter.len()] == *delimiter {
            if i > start {
                chunks.push(&buffer[start..i]);
            }
            start = i + delimiter.len();
        }
    }
    if start < buffer.len() {
        chunks.push(&buffer[start..]);
    }
    chunks
}

fn process_chunk(chunk: &[u8], output_dir: &str) -> Result<(), String> {
    if chunk.len() < 3 {
        return Ok(());
    }

    let buffer_size = u16::from_be_bytes([chunk[16], chunk[15]]) as usize;
    if buffer_size == 0 {
        return Ok(());
    }
    let string_length = chunk[21] as usize;

    if chunk.len() < 3 + string_length {
        return Ok(());
    }

    let filename_bytes = &chunk[25..25 + string_length];
    let filename = String::from_utf8_lossy(filename_bytes).to_string();
    let buffer_data = &chunk[25 + string_length..];

    let output_path = Path::new(output_dir).join(&filename);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    println!("filename: {}, buffer: {}", filename, buffer_size);
    let mut output_file = File::create(&output_path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    output_file
        .write_all(buffer_data)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let delimiter = [0x47, 0x42, 0x4D, 0x50, 0x0A];

    let input_path = validate_input(&args)?;

    let output_dir = Path::new(&input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or("output".to_string());
    fs::create_dir_all(&output_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    let buffer = read_and_validate_file(input_path, &delimiter)?;

    let chunks = split_chunks(&buffer, &delimiter);

    for chunk in chunks {
        process_chunk(chunk, &output_dir)?;
    }

    Ok(())
}