use crate::load::UM;

/// Loads the value at $m[$r[B]][$r[C]] into $r[A]
/// # Arguments
/// * a - the value of register A
/// * b - the value of register B
/// * c - the value of register C
/// * um - the current state of the universal machine
pub fn segmented_load(a: usize, b: usize, c: usize, um: &mut UM) {
    um.registers[a] = um.memory[um.registers[b] as usize][um.registers[c] as usize];
    um.counter += 1;
}

/// Loads the value at $r[C] into $m[$r[A]][$r[B]]
/// # Arguments
/// * a - the value of register A
/// * b - the value of register B
/// * c - the value of register C
/// * um - the current state of the universal machine
pub fn segmented_store(a: usize, b: usize, c: usize, um: &mut UM) {
    um.memory[um.registers[a] as usize][um.registers[b] as usize] = um.registers[c];
    um.counter += 1;
}

/// Creates a new memory segment with a number of words equal to the value in $r[C].
/// The function will use any unmapped memory segment available, and if there are none
/// it will create a new segment at the end of the memory vector.
/// # Arguments
/// * b - the value of register B
/// * c - the value of register C
/// * um - the current state of the universal machine
pub fn map(b: usize, c: usize, um: &mut UM) {
    um.counter += 1;
    if !um.unmapped.is_empty() {
        um.registers[b] = um.unmapped.pop().unwrap();
        um.memory[um.registers[b] as usize] = vec![0; um.registers[c] as usize];
        return;
    }
    um.memory.push(vec![0; um.registers[c] as usize]);
    um.registers[b] = (um.memory.len() - 1) as u32;
}

/// Clears the memory segment at $m[$r[C]] and pushes the index into the unmapped vector
/// # Arguments
/// * c - the value of register C
/// * um - the current state of the universal machine
pub fn unmap(c: usize, um: &mut UM) {
    um.unmapped.push(um.registers[c]);
    um.memory[um.registers[c] as usize].clear();
    um.counter += 1;
}

/// Loads a value into $r[A].
/// # Arguments
/// * a - the value of register A
/// * val - the value to be loaded into register A
/// * um - the current state of the universal machine
pub fn loadv(a: usize, val: u32, um: &mut UM) {
    let new_val = 0x1ffffff & val;
    um.registers[a] = new_val;
    um.counter += 1;
}

/// Duplicates the segment $m[$r[B]] and replaces $m[0] with the duplicated segment. The counter
/// is set to point to $m[0][$r[C]]
/// # Arguments
/// * b - the value of register B
/// * c - the value of register C
/// * um - the current state of the universal machine
pub fn loadp(b: usize, c: usize, um: &mut UM) {
    if um.registers[b] != 0 {
        um.memory[0] = um.memory[um.registers[b] as usize].clone();
    }
    um.counter = um.registers[c];
}
