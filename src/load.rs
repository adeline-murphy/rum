use crate::bitshift::{get, op, RA, RB, RC, RL, VL};
use crate::io::{input, output};
use crate::memory::{loadp, loadv, map, segmented_load, segmented_store, unmap};
use crate::operations::{addition, bitwise_nand, conditional_move, division, halt, multiplication};
use num_derive::FromPrimitive;
use std::process::exit;

#[derive(FromPrimitive)]
#[repr(u32)]
pub enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    Alloc,
    Free,
    Output,
    Input,
    LoadP,
    LoadV,
}

/// Represents the Universal Machine with a two-dimensional vector of memory segments, a vector
/// of unmapped indices, 8 32-bit registers in a raw array and a 32-bit program counter
#[derive(Debug, PartialEq, Clone)]
pub struct UM {
    pub memory: Vec<Vec<u32>>,
    pub unmapped: Vec<u32>,
    pub registers: [u32; 8],
    pub counter: u32,
}

/// Loads the inputted bytes into a vector of u32 values that represent the instructions for the
/// Universal Machine.
/// # Arguments
/// * input - the name of the binary file to read
pub fn memory_load(input: Option<&str>) -> Vec<u32> {
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(
            std::fs::File::open(filename).unwrap(),
        )),
    };
    let mut buf = Vec::<u8>::new();
    raw_reader.read_to_end(&mut buf).unwrap();
    let instructions: Vec<u32> = buf
        .chunks_exact(4)
        .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
        .collect();
    instructions
}

/// Drives the execution of the Universal Machine by creating the UM and matching each opcode instruction to its
/// corresponding function call.
/// # Arguments
/// * filename: the name of the binary file to read
pub fn execute(filename: Option<&str>) {
    let instructions = memory_load(filename.as_deref());
    let mut um = UM {
        memory: vec![instructions],
        unmapped: Vec::new(),
        registers: [0; 8],
        counter: 0,
    };
    loop {
        let instruction = um.memory[0][um.counter as usize];
        match op(instruction) {
            Some(Opcode::CMov) => {
                let a = get(&RA, instruction) as usize;
                let b = get(&RB, instruction) as usize;
                let c = get(&RC, instruction) as usize;
                conditional_move(a, b, c, &mut um);
            }
            Some(Opcode::Load) => {
                let a = get(&RA, instruction) as usize;
                let b = get(&RB, instruction) as usize;
                let c = get(&RC, instruction) as usize;
                segmented_load(a, b, c, &mut um);
            }
            Some(Opcode::Store) => {
                let a = get(&RA, instruction) as usize;
                let b = get(&RB, instruction) as usize;
                let c = get(&RC, instruction) as usize;
                segmented_store(a, b, c, &mut um);
            }
            Some(Opcode::Add) => {
                let a = get(&RA, instruction) as usize;
                let b = get(&RB, instruction) as usize;
                let c = get(&RC, instruction) as usize;
                addition(a, b, c, &mut um);
            }
            Some(Opcode::Mul) => {
                let a = get(&RA, instruction) as usize;
                let b = get(&RB, instruction) as usize;
                let c = get(&RC, instruction) as usize;
                multiplication(a, b, c, &mut um);
            }
            Some(Opcode::Div) => {
                let a = get(&RA, instruction) as usize;
                let b = get(&RB, instruction) as usize;
                let c = get(&RC, instruction) as usize;
                division(a, b, c, &mut um);
            }
            Some(Opcode::Nand) => {
                let a = get(&RA, instruction) as usize;
                let b = get(&RB, instruction) as usize;
                let c = get(&RC, instruction) as usize;
                bitwise_nand(a, b, c, &mut um);
            }
            Some(Opcode::Halt) => {
                halt();
            }
            Some(Opcode::Alloc) => {
                let b = get(&RB, instruction) as usize;
                let c = get(&RC, instruction) as usize;
                map(b, c, &mut um);
            }
            Some(Opcode::Free) => {
                let c = get(&RC, instruction) as usize;
                unmap(c, &mut um);
            }
            Some(Opcode::Output) => {
                let c = get(&RC, instruction) as usize;
                output(c, &mut um);
            }
            Some(Opcode::Input) => {
                let c = get(&RC, instruction) as usize;
                input(c, &mut um);
            }
            Some(Opcode::LoadP) => {
                let b = get(&RB, instruction) as usize;
                let c = get(&RC, instruction) as usize;
                loadp(b, c, &mut um);
            }
            Some(Opcode::LoadV) => {
                let a = get(&RL, instruction) as usize;
                let val = get(&VL, instruction);
                loadv(a, val, &mut um);
            }
            _ => exit(1),
        }
    }
}
