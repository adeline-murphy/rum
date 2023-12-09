// use rum::operations;
use rum::load::execute;
use std::env;

fn main() {
    let input = env::args().nth(1);
    execute(input.as_deref());
}
