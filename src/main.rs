use std::fs::File;
use std::io::prelude::*;
#[derive(Debug)]
struct MyStruct {
    data: Vec<u8>,
}
fn main() -> std::io::Result<()>{
    let delimiter = vec![0x47,0x42,0x4D,0x50,0x0A];
    let path = String::from(r#"C:\hocPhone\data\res.bin"#);
    println!("{}" ,path);
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut chunks: Vec<Vec<u8>> = Vec::new();
    let mut current_chunk = Vec::new();
    let mut i = 0;
    while i < buffer.len() {
        if i <= buffer.len() - delimiter.len() 
        && buffer[i..i + delimiter.len()] == delimiter[..] {
            if !current_chunk.is_empty() {
                chunks.push(current_chunk);
                current_chunk = Vec::new();
            }
            i += delimiter.len();
        } else {
            current_chunk.push(buffer[i]);
            i += 1;
        }
    } 
    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    let structs: Vec<MyStruct> = chunks.into_iter()
        .map(|chunk| {
            MyStruct { data: chunk }
        })
        .collect();

    println!("Found {} chunks", structs.len());
    for (i, s) in structs.iter().enumerate() {
        for  j in s.data.iter().take(20)
        {
            print!("{:02X?}" ,j);
        }
        println!("Chunk {}: {} bytes", i, s.data.len());
    }

    Ok(())
}
