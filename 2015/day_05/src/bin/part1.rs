use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

trait IsNice {
    fn is_nice(&self) -> bool;
}

#[derive(Debug, Default)]
struct Checklist {
    vowels_count : u32,
    has_twins : bool,
    has_bad_string : bool,
}

impl IsNice for Checklist {
    fn is_nice(&self) -> bool {
        self.vowels_count >= 3 && self.has_twins && !self.has_bad_string
    }
}

fn part1(input:&str) -> String{
    let bad_strings : Vec<&str> = vec!["ab", "cd","pq","xy"];
    let vowels = vec!['a','e','i','o','u'];
    let mut nice = 0;
    for line in input.lines(){
        let mut checklist = Checklist{..Default::default()};

        if vowels.contains(&line.chars().nth(0).unwrap()) {
            checklist.vowels_count += 1;
        }

        for (c1,c2) in line.chars().tuple_windows() {
            if c1 == c2 {
                checklist.has_twins = true;
            }
            if bad_strings.contains(&format!("{c1}{c2}").as_str()) {
                checklist.has_bad_string = true;
                break;
            }
            if vowels.contains(&c2)  {
                checklist.vowels_count += 1;
            }
        }
        if checklist.is_nice() {
            nice += 1;
        }
    }
    nice.to_string()
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_1() {
        let result = part1("ugknbfddgicrmopn");
        assert_eq!(result, "1");
        let result = part1("aaa");
        assert_eq!(result, "1");
        let result = part1("jchzalrnumimnmhp");
        assert_eq!(result, "0");
        let result = part1("haegwjzuvuyypxyu");
        assert_eq!(result, "0");
        let result = part1("dvszwmarrgswjxmb");
        assert_eq!(result, "0");
    }
}
