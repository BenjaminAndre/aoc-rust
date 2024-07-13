#[aoc::main(2015, 01)]
fn main(input: &str) -> (usize, usize) {
    let positive = input.chars().filter(|c| *c == '(').count();
    let negative = input.chars().filter(|c| *c == ')').count();
    (positive - negative, 0)
}
