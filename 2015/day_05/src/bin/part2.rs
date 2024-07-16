use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

trait IsNice {
    fn is_nice(&self) -> bool;
}

#[derive(Debug, Default)]
struct Checklist {
    pairs : HashMap<String, u32>,
    has_sandwich : bool,
}

impl IsNice for Checklist {
    fn is_nice(&self) -> bool {
        self.has_sandwich && self.pairs.values().any(|v| *v >= 2)
    }
}

fn part2(input:&str) -> String{
    let mut nice = 0;
    // It works because none of ,;: are in the input string. Not quite idiomatic, but it works
    for line in input.lines().map(|l| format!("{},;:", l)) {
        let mut checklist = Checklist{..Default::default()};

        for (c1,c2,c3,c4) in line.chars().tuple_windows() {

            if c1 == c3 {
                checklist.has_sandwich = true;
            }
            let pair = format!("{}{}", c1,c2);
            let antipair = format!("{}{}", c2,c3);
            let lastpair = format!("{}{}", c3,c4);
            if antipair != pair || pair == lastpair {
                checklist.pairs.entry(pair).and_modify(|v| *v+=1).or_insert(1);
            }
            if checklist.is_nice() {
                nice += 1;
                break;
            }
        }
    }
    nice.to_string()
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let result = part2("qjhvhtzxzqqjkmpb");
        assert_eq!(result, "1");
        let result = part2("xxyxx");
        assert_eq!(result, "1");
        let result = part2("uurcxstgmygtbstg");
        assert_eq!(result, "0");
        let result = part2("ieodomkazucvgmuy");
        assert_eq!(result, "0");
        let result = part2("aaa");
        assert_eq!(result, "0");
        let result = part2("aaaa");
        assert_eq!(result, "1");
    }
}
