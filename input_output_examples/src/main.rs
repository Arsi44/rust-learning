// use std::io;
use std::io::{self, BufReader, BufRead, SeekFrom, Write};
use std::io::prelude::*;
use std::fs::File;

// // Write example
// fn main() -> std::io::Result<()> {
//     let data = b"some bytes";

//     let mut pos = 0;
//     let mut buffer = File::create("foo.txt")?;

//     while pos < data.len() {
//         let bytes_written = buffer.write(&data[pos..])?;
//         pos += bytes_written;
//     }
//     Ok(())
// }

// // Read example
// fn main() -> io::Result<()> {

//     let mut f = File::open("foo.txt")?;
//     let mut buffer = [0; 10];

//     // read up to 10 bytes
//     f.read(&mut buffer)?;

//     let mut buffer = Vec::new();
//     // read the whole file
//     f.read_to_end(&mut buffer)?;

//     // read into a String, so that you don't need to do the conversion.
//     let mut buffer = String::new();
//     f.read_to_string(&mut buffer)?;

//     // and more! See the other methods for more details.
//     Ok(())
// }

// // BufReader example
// fn main() -> io::Result<()> {
//     let f = File::open("foo.txt")?;
//     let f = BufReader::new(f);

//     for line in f.lines() {
//         println!("{}", line.unwrap());
//     }

//     Ok(())
// }

// // Seek example
// fn main() -> io::Result<()> {
//     let mut f = File::open("foo.txt")?;

//     // move the cursor 42 bytes from the start of the file
//     f.seek(SeekFrom::Start(42))?;
//     Ok(())
// }

// // stdin() example
// fn main() -> io::Result<()> {
//     let mut buffer = String::new();
//     let stdin = io::stdin();
//     let mut handle = stdin.lock();

//     handle.read_line(&mut buffer)?;
//     Ok(())
// }

// // stdout() example
// fn main() -> io::Result<()> {
//     let stdout = io::stdout();
//     let mut handle = stdout.lock();

//     handle.write_all(b"hello world")?;

//     Ok(())
// }

// stderr() example
fn main() -> io::Result<()> {
    let stderr = io::stderr();
    let mut handle = stderr.lock();

    handle.write_all(b"hello world")?;

    Ok(())
}
