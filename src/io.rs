use crate::load::UM;
use std::io;
use std::io::{Read};

/// Immediately outputs the value in $r[C] to standard out.
/// # Arguments
/// * c - the value of register C
/// * um - the current state of the universal machine
pub fn output(c: usize, um: &mut UM) {
    um.counter += 1;
    print!("{}", char::from_u32(um.registers[c]).unwrap());
}

/// Reads an input from standard in and loads it into $r[C]. If the end of input is signalled,
/// $r[C] is loaded with a u32 bit word in which every bit is 1.
/// # Arguments
/// * c - the value of register C
/// * um - the current state of the universal machine
pub fn input(c: usize, um: &mut UM) {
    um.counter += 1;
    let stdin = io::stdin();
    let input_byte = stdin.bytes().next();
    match input_byte {
        Some(Ok(byte)) => {
            um.registers[c] = byte as u32;
        }
        Some(Err(err)) => {
            eprintln!("Error reading input: {}", err);
        }
        None => {
            um.registers[c] = u32::MAX;
        }
    }
}
