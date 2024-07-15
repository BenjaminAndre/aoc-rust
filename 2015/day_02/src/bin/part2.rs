use std::cmp;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input:&str) -> String{
    let output : i32 = input.lines().map(|row| {
        let vals : Vec<i32> = row.split('x').map(|x| x.parse::<i32>().unwrap()).take(3).collect();
        let side1 = vals[0] + vals[1];
        let side2 = vals[1] + vals[2];
        let side3 = vals[0] + vals[2];
        let minimal = cmp::min(side1, cmp::min(side2, side3));
        2*minimal + vals[0]*vals[1]*vals[2]

    }).sum();
    output.to_string()
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_1() {
        let result = part1("2x3x4");
        assert_eq!(result, "34");
        let result = part1("1x1x10");
        assert_eq!(result, "14");
    }
}
