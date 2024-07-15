fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut floor : i32 = 0;
    for (id,c) in input.chars().enumerate() {
        match c {
            '(' => floor+=1,
            _ => floor-=1,
        };
        if floor < 0 {
            return (id+1).to_string();
        }
    }
    input.chars().count().to_string()
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_1() {
        let result = part2(")");
        assert_eq!(result, "1");
        let result = part2("()())");
        assert_eq!(result, "5");
    }

    #[test]
    fn test_2(){
        let result = part2(include_str!("../../input1.txt"));
        assert_eq!(result, "1795");

    }

}
