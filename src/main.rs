use std::{fs, path::Path};

use args::{DecodeArgs, EncodeArgs, PngMeArgs, PrintArgs, RemoveArgs};
use chunk::Chunk;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod png;

fn handle_encode(cmd: EncodeArgs) -> std::io::Result<()> {
    let path = &cmd.file_path;
    let bytes = fs::read(path)?;
    let mut img = png::Png::try_from(bytes.as_slice()).unwrap();

    let new_chunk = Chunk::new(cmd.chunk_type, cmd.message.into_bytes());

    img.append_chunk(new_chunk);

    let output_path = cmd.output_file.as_ref().unwrap_or(&cmd.file_path);

    fs::write(output_path, img.as_bytes())?;

    // Write updated PNG back to file
    fs::write(output_path, img.as_bytes())?;

    println!("Chunk encoded successfully. Try decoding or printing the output file.");

    Ok(())
}

fn handle_decode(cmd: DecodeArgs) -> std::io::Result<()> {
    let path = &cmd.file_path;
    let bytes = fs::read(path)?;
    let img = png::Png::try_from(bytes.as_slice()).unwrap();

    let chunk_type_clone = cmd.chunk_type.clone().bytes();
    let chunk_type_str = std::str::from_utf8(&chunk_type_clone).unwrap();

    match img.chunk_by_type(chunk_type_str) {
        Some(chunk) => {
            println!("Encoded chunk is: {chunk}");
        }
        None => {
            println!("This chunk was not found in the file.");
        }
    }

    Ok(())
}

fn handle_remove(cmd: RemoveArgs) -> std::io::Result<()> {
    let path = &cmd.file_path;
    let bytes = fs::read(path)?;
    let mut img = png::Png::try_from(bytes.as_slice()).unwrap();

    let chunk_type_clone = cmd.chunk_type.clone().bytes();
    let chunk_type_str = std::str::from_utf8(&chunk_type_clone).unwrap();

    match img.remove_first_chunk(chunk_type_str) {
        Ok(chunk) => {
            println!("Encoded chunk was removed: {chunk}");
        }
        Err(e) => {
            println!("This chunk was not found in the file {e}");
        }
    }

    fs::write(path, img.as_bytes())?;

    Ok(())
}

fn handle_print(cmd: PrintArgs) -> std::io::Result<()> {
    let path = &cmd.file_path;
    let bytes = fs::read(path)?;
    let img = png::Png::try_from(bytes.as_slice()).unwrap();

    println!("Your file: {img}");

    Ok(())
}

fn main() {
    let args = PngMeArgs::parse();

    match args {
        PngMeArgs::Encode(cmd) => {
            handle_encode(cmd).unwrap();
        }
        PngMeArgs::Decode(cmd) => {
            handle_decode(cmd).unwrap();
        }

        PngMeArgs::Remove(cmd) => {
            handle_remove(cmd).unwrap();
        }
        PngMeArgs::Print(cmd) => {
            handle_print(cmd).unwrap();
        }
    }
}
