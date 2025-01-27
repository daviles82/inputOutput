use std::fs::OpenOptions;
use std::io::{self, Read, Write, Seek, SeekFrom};

fn main() -> io::Result<()> {
    let mut basic_file = OpenOptions::new()
    .read(true).write(true).append(true).open("textfile.txt")?;

    // Data structure "String", "new()" initializes an instance of a String data structure
    let mut program_lang = String::new();


    // write (expression) b = byte string literal
    basic_file.write_all(b"\ncss")?;

    // find the last text (expression)
    basic_file.seek(SeekFrom::Start(0))?;

    // read (expression)
    basic_file.read_to_string(&mut program_lang)?;
    println!("What the file contains: \n{}", program_lang);

    // returns closure (expression)
    Ok(())
}

