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
    let mut current_boy_santa = GoodBoy(0,0);
    let mut current_boy_robot = GoodBoy(0,0);

    stops.insert(current_boy_santa.clone());

    for (id, c) in input.chars().enumerate() {
        let clone : GoodBoy; 
        if id %2 == 0 {
            current_boy_santa = match c {
                '^' => GoodBoy(current_boy_santa.0, current_boy_santa.1 + 1),
                '>' => GoodBoy(current_boy_santa.0 + 1, current_boy_santa.1),
                'v' => GoodBoy(current_boy_santa.0, current_boy_santa.1 - 1),
                '<' => GoodBoy(current_boy_santa.0 - 1, current_boy_santa.1),
                '\n' => current_boy_santa,
                _a => panic!("Unexpected symbol in input : {:?} !", _a)
            };
            clone = current_boy_santa.clone();
        } else {
            current_boy_robot = match c {
                '^' => GoodBoy(current_boy_robot.0, current_boy_robot.1 + 1),
                '>' => GoodBoy(current_boy_robot.0 + 1, current_boy_robot.1),
                'v' => GoodBoy(current_boy_robot.0, current_boy_robot.1 - 1),
                '<' => GoodBoy(current_boy_robot.0 - 1, current_boy_robot.1),
                '\n' => current_boy_robot,
                _a => panic!("Unexpected symbol in input : {:?} !", _a)
            };
            clone = current_boy_robot.clone();
        }

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
        let result = part1("^v");
        assert_eq!(result, "3");
        let result = part1("^>v<");
        assert_eq!(result, "3");
        let result = part1("^v^v^v^v^v");
        assert_eq!(result, "11");
    }
}
