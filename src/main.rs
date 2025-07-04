use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input_paths: Vec<String>,
}

fn validate_input(args: &[String]) -> Result<Vec<String>, String> {
    if args.len() != 2 {
        return Err(format!("Usage: {} <input_path>", args[0]));
    }
    let input_path = Path::new(&args[1]);

    if input_path.is_dir() {
        let bin_files: Vec<String> = fs::read_dir(input_path)
            .map_err(|e| format!("Failed to read directory {}: {e}", input_path.display()))?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("bin") {
                    path.to_str().map(|s| s.to_string())
                } else {
                    None
                }
            })
            .collect();

        if bin_files.is_empty() {
            return Err("No .bin files found in directory".to_string());
        }
        Ok(bin_files)
    } else {
        if input_path.extension().and_then(|s| s.to_str()) != Some("bin") {
            return Err("Input file must have .bin extension".to_string());
        }
        Ok(vec![args[1].clone()])
    }
}

fn read_and_validate_file(path: &str, delimiter: &[u8]) -> Result<Vec<u8>, String> {
    let mut file = File::open(path).map_err(|e| format!("Failed to open file {path}: {e}"))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read file {path}: {e}"))?;

    if buffer.len() < delimiter.len() || buffer[0..delimiter.len()] != *delimiter {
        return Err(format!("File {path} does not start with valid delimiter"));
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
    if chunk.len() < 17 {
        return Ok(());
    }
    let buffer_size = u32::from_be_bytes([chunk[17], chunk[16], chunk[15], chunk[14]]) as usize;
    if buffer_size == 0 {
        return Ok(());
    }
    let string_length = chunk[22] as usize;

    if chunk.len() < 3 + string_length {
        return Ok(());
    }

    let filename_bytes = &chunk[26..26 + string_length];
    let filename = String::from_utf8_lossy(filename_bytes).to_string();
    let buffer_data = &chunk[26 + string_length..];

    let output_path = Path::new(output_dir).join(&filename);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
    }

    println!("filename: {filename}, buffer: {buffer_size}");
    let mut output_file = File::create(&output_path)
        .map_err(|e| format!("Failed to create file {}: {}", output_path.display(), e))?;
    output_file
        .write_all(buffer_data)
        .map_err(|e| format!("Failed to write file {}: {}", output_path.display(), e))?;

    Ok(())
}

fn process_file(input_path: &str, delimiter: &[u8]) -> Result<(), String> {
    let output_dir = Path::new("Output").join(
        Path::new(&input_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
            .unwrap_or("output".to_string()),
    );
    let output_dir = output_dir
        .to_str()
        .ok_or_else(|| format!("Invalid output directory path for {input_path}"))?;

    fs::create_dir_all(output_dir)
        .map_err(|e| format!("Failed to create output directory {output_dir}: {e}"))?;

    let buffer = read_and_validate_file(input_path, delimiter)?;

    let chunks = split_chunks(&buffer, delimiter);

    for chunk in chunks {
        process_chunk(chunk, output_dir)?;
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let delimiter = [0x47, 0x42, 0x4D, 0x50];
    let args = Args::parse_from(args);

    let input_paths = validate_input(&args.input_paths)?;

    for input_path in input_paths {
        println!("Processing file: {input_path}");
        if let Err(e) = process_file(&input_path, &delimiter) {
            eprintln!("Error processing file {input_path}: {e}");
            continue;
        }
    }

    Ok(())
}
