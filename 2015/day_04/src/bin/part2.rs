use crypto::md5::Md5;
use crypto::digest::Digest;


fn main() {
    let output = part1(include_str!("../../input1.txt").trim());
    dbg!(output);
}

fn part1(input:&str) -> String{
    let desired_prefix = "0".repeat(6);
    let mut md5 = Md5::new();
    let result = (0..).filter(|x| {
        md5.reset();
        let inp = format!("{}{}", input, x);
        let inp = inp.as_bytes();
        md5.input(&inp);
        let res = md5.result_str();
        res.starts_with(&desired_prefix)
    }).nth(0).unwrap();
    result.to_string()
}

