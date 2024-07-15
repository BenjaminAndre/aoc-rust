fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input:&str) -> String{
    let up = input.chars().filter(|c| *c == '(').count();
    let down = input.chars().filter(|c| *c == ')').count();
    (up-down).to_string()
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_1() {
        let result = part1("(())");
        assert_eq!(result, "0");
        let result = part1("()()");
        assert_eq!(result, "0");
        let result = part1("(((");
        assert_eq!(result, "3");
        let result = part1("(()(()(");
        assert_eq!(result, "3");
        let result = part1("))(((((");
        assert_eq!(result, "3");
        let result = part1("())");
        assert_eq!(result, "-1");
        let result = part1("))(");
        assert_eq!(result, "-1");
        let result = part1(")))");
        assert_eq!(result, "-3");
        let result = part1(")())())");
        assert_eq!(result, "-3");
    }
}
