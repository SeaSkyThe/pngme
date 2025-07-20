use std::path::PathBuf;

use clap::Parser;

use crate::chunk_type::ChunkType;

#[derive(Parser, Debug)]
#[command(version, long_about = None, color = clap::ColorChoice::Auto, about = "Encode messages in PNG files")]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
#[command(about = "Encode a message with a ChunkType")]
pub struct EncodeArgs {
    #[arg(help = "Path to the input PNG file")]
    pub file_path: PathBuf,
    #[arg(help = "4-character chunk type (e.g 'ruST'), this is your message 'key' you will use it to recover the message")]
    pub chunk_type: ChunkType,
    #[arg(help = "The message to encode in the PNG file")]
    pub message: String,
    #[arg(help = "Optional output file, if you don't pass any, the input file will be updated.")]
    pub output_file: Option<PathBuf>,
}

#[derive(Parser, Debug)]
#[command(version, about = "Recovers the first message with a ChunkType", long_about = None)]
pub struct DecodeArgs {
    #[arg(help = "Path to the PNG file that contains the message")]
    pub file_path: PathBuf,
    #[arg(help = "The chunk type (message key)")]
    pub chunk_type: ChunkType,
}

#[derive(Parser, Debug)]
#[command(version, about = "Removes the first chunk with a specific ChunkType from a PNG file", long_about = None)]
pub struct RemoveArgs {
    #[arg(help = "Path to the PNG file that will have the message removed")]
    pub file_path: PathBuf,
    #[arg(help = "The chunk type (message key)")]
    pub chunk_type: ChunkType,
}

#[derive(Parser, Debug)]
#[command(version, about = "Print all chunks", long_about = None)]
pub struct PrintArgs {
    #[arg(help = "Path for the PNG file")]
    pub file_path: PathBuf,
}
