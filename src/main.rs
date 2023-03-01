use std::env::args;
use std::io::{Result, Read, BufReader};
use std::fs::File;


fn main() -> Result<()> {
    let mut position = -1;
    let name = match args().nth(1) {
        None    => String::from("KYLE"),
        Some(n) => n.to_uppercase(),
    };
    let mut digits: Vec<u8> = Vec::new();
    for digit in name.chars() {
        digits.push(digit as u8);
    }
    let f = File::open("../base26-pi/pi-b26.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;
    for value in buffer.windows(digits.len()) {
        if value == digits {
            println!("found: {:?} at position {:?}", name, position);
            return Ok(());
        }
        position = position + 1;
    }
    println!("{} was not found in {} digits of Pi.", name, buffer.len());
    Ok(())
}
