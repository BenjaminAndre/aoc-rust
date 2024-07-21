use crypto::md5::Md5;
use crypto::digest::Digest;


fn main() {
    let output = part1(include_str!("../../input1.txt").trim());
    dbg!(output);
}

fn part1(input:&str) -> String{
    let desired_prefix = "0".repeat(5);
    let mut md5 = Md5::new();
    let result = (0..).filter(|x| {
        md5.reset();
        let inp = format!("{}{}", input, x);
        let inp = inp.as_bytes();
        md5.input(&inp);
        let res = md5.result_str();
        if *x > 609040 && *x < 609050 {
        }
        res.starts_with(&desired_prefix)
    }).nth(0).unwrap();
    result.to_string()
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_1() {
        let result = part1("abcdef");
        assert_eq!(result, "609043");
    }

    #[test]
    fn test_2(){
        let result = part1("pqrstuv");
        assert_eq!(result, "1048970");
    }
}
