use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
struct GoodBoy(i32,i32);


fn part1(input:&str) -> String{

    let mut stops = HashSet::new();
    let mut current_boy = GoodBoy(0,0);

    stops.insert(current_boy.clone());
    for c in input.chars() {
        current_boy = match c {
            '^' => GoodBoy(current_boy.0, current_boy.1 + 1),
            '>' => GoodBoy(current_boy.0 + 1, current_boy.1),
            'v' => GoodBoy(current_boy.0, current_boy.1 - 1),
            '<' => GoodBoy(current_boy.0 - 1, current_boy.1),
            '\n' => current_boy,
            _a => panic!("Unexpected symbol in input : {:?} !", _a)
        };
        let clone = current_boy.clone();
        if !stops.contains(&clone) {
            stops.insert(clone);
        }
    }
    stops.len().to_string()
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_1() {
        let result = part1(">");
        assert_eq!(result, "2");
        let result = part1("^>v<");
        assert_eq!(result, "4");
        let result = part1("^v^v^v^v^v");
        assert_eq!(result, "2");
    }
}
