use crate::load::UM;
use std::process::exit;

/// Sets $r[A] equal to $r[B] iff the value at $r[C] is 0
/// # Arguments
/// * a - the value of register A
/// * b - the value of register B
/// * c - the value of register C
/// * um - the current state of the universal machine
pub fn conditional_move(a: usize, b: usize, c: usize, um: &mut UM) {
    if um.registers[c] != 0 {
        um.registers[a] = um.registers[b];
    }
    um.counter += 1;
}

/// Adds the contents of $r[B] and $r[C] together and stores the result in $r[A]
/// # Arguments
/// * a - the value of register A
/// * b - the value of register B
/// * c - the value of register C
/// * um - the current state of the universal machine
pub fn addition(a: usize, b: usize, c: usize, um: &mut UM) {
    um.registers[a] = ((um.registers[b] as u64 + um.registers[c] as u64) % u64::pow(2, 32)) as u32;
    um.counter += 1;
}

/// Multiplies the contents of $r[B] and $r[C] together and stores the result in $r[A]
/// # Arguments
/// * a - the value of register A
/// * b - the value of register B
/// * c - the value of register C
/// * um - the current state of the universal machine
pub fn multiplication(a: usize, b: usize, c: usize, um: &mut UM) {
    um.registers[a] = ((um.registers[b] as u64 * um.registers[c] as u64) % u64::pow(2, 32)) as u32;
    um.counter += 1;
}

/// Divides the contents of $r[B] by the contents of $r[C] and stores the result in $r[A]
/// # Arguments
/// * a - the value of register A
/// * b - the value of register B
/// * c - the value of register C
/// * um - the current state of the universal machine
pub fn division(a: usize, b: usize, c: usize, um: &mut UM) {
    um.registers[a] = um.registers[b] / um.registers[c];
    um.counter += 1;
}

/// Performs bitwise nand on $r[B] and $r[C] and stores the result in $r[A]
/// # Arguments
/// * a - the value of register A
/// * b - the value of register B
/// * c - the value of register C
/// * um - the current state of the universal machine
pub fn bitwise_nand(a: usize, b: usize, c: usize, um: &mut UM) {
    um.registers[a] = !(um.registers[b] & um.registers[c]);
    um.counter += 1;
}

/// Halts execution of the program by exiting with code 0
pub fn halt() {
    exit(0);
}
