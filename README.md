# PNGme

A command-line tool for encoding, decoding, removing and message into PNG files as PNG chunks.

## Prerequisites

Make sure you have Rust and Cargo installed. You can start from [rustup.rs](https://rustup.rs/).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Build the project

Clone the project and navigate to the project folder:

```bash
git clone https://github.com/SeaSkyThe/pngme.git
cd pngme
```

You can just use `cargo run` for quick testing:

```bash
cargo run -- <SUBCOMMAND> [OPTIONS]
```

Or you can compile the project in release mode and use the binary:

```bash
cargo build --release
./target/release/pngme <SUBCOMMAND> [OPTIONS]
```

## Available commands

```
Usage: pngme <COMMAND>

Commands:
  encode  Encode a message with a ChunkType
  decode  Recovers the first message with a ChunkType
  remove  Removes the first chunk with a specific ChunkType from a PNG file
  print   Print all chunks
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Encode

```
Usage: pngme encode <FILE_PATH> <CHUNK_TYPE> <MESSAGE> [OUTPUT_FILE]

Arguments:
  <FILE_PATH>    Path to the input PNG file
  <CHUNK_TYPE>   4-character chunk type (e.g 'ruST'), this is your message 'key'

  <MESSAGE>      The message to encode in the PNG file
  [OUTPUT_FILE]  Optional output file, if you don't pass any, the input file will be updated.

Options:
  -h, --help  Print help
```

### Decode

```
Recovers the first message with a ChunkType

Usage: pngme decode <FILE_PATH> <CHUNK_TYPE>

Arguments:
  <FILE_PATH>   Path to the PNG file that contains the message
  <CHUNK_TYPE>  The chunk type (message key)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Remove

```
Removes the first chunk with a specific ChunkType from a PNG file

Usage: pngme remove <FILE_PATH> <CHUNK_TYPE>

Arguments:
  <FILE_PATH>   Path to the PNG file that will have the message removed
  <CHUNK_TYPE>  The chunk type (message key)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Print 

```

Print all chunks

Usage: pngme print <FILE_PATH>

Arguments:
  <FILE_PATH>  Path for the PNG file

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Example

Here is a simple flow of usage: 

#### 1.  Encode a message into a PNG
Save a message into a new PNG file:

 ```bash
cargo run -- encode ./dice.png ruSt "This is our message" ./output.png
```
> This embeds the message "This is our message" in a custom chunk of type ruSt into dice.png, saving the result to output.png.

#### 2. Decode a hidden message: 
Read the hidden message from the PNG:

```bash
cargo run -- decode ./output.png ruSt
```
> If the chunk exists, the message will be printed to the terminal.


#### 3. Remove a message:
Delete the chunk of type `ruSt`:

```bash
cargo run -- remove ./output.png ruSt
```
> If the chunk exists, the message will be printed to the terminal.


#### 4. Try decoding again
Attempt to read the message after deletion:

```bash
cargo run -- decode ./output.png ruSt
```

> You should see an error or an empty result, indicating the chunk is no longer present.

#### 5. View all PNG chunks
List all chunks (standard and custom) in the PNG file:

```bash
cargo run -- print ./output.png
```

> This will show chunk types and sizes, useful for verifying embedded data.

