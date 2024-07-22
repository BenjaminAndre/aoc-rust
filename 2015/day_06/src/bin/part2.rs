use regex::Regex;
use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
struct Point (usize,usize);


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Order {
    TurnOn(Point, Point),
    TurnOff(Point, Point),
    Toggle(Point, Point),
}


fn parse(input:&str) -> Vec<Order> {
    let mut orders = Vec::new(); 
    let re = Regex::new(r"\d+").unwrap(); // \d means digit
    for line in input.lines(){
        let mut p1 : Option<Point> = None;
        let mut p2 : Option<Point> = None;

        for (match1, match2) in re.find_iter(line).tuples() {
            let n1 = &line[match1.start()..match1.end()].parse().unwrap();
            let n2 = &line[match2.start()..match2.end()].parse().unwrap();
            if p1 == None {
                p1 = Some(Point(*n1, *n2));
            } else {
                p2 = Some(Point(*n1, *n2));
            }
        }

        let p1 = p1.unwrap();
        let p2 = p2.unwrap();

        if line.contains("off") {
            orders.push(Order::TurnOff(p1, p2));
        } else if line.contains("on") {
            orders.push(Order::TurnOn(p1, p2));
        } else if line.contains("toggle") {
            orders.push(Order::Toggle(p1, p2));
        }
    }
    orders
}

fn process<F>(p1 : Point, p2: Point, lights : &mut[u8; 1000000], action : F)
where F : Fn(u8) -> u8
{
    for y in p1.1..=p2.1 {
        for x in p1.0..=p2.0 {
            let index = y + x * 1000;
            lights[index] = action(lights[index]);
        }
    }
}

fn part1(input:&str) -> String{
    let orders = parse(input);
    let mut lights : [u8; 1000000] = [0; 1000000];

    for order in orders.iter() {
        match order {
            Order::TurnOn(p1,p2) => {process(*p1,*p2, &mut lights, |i| i+1)},
            Order::TurnOff(p1,p2) => {process(*p1,*p2, &mut lights, |i| i.checked_sub(1).unwrap_or(0))},
            Order::Toggle(p1,p2) => {process(*p1,*p2, &mut lights, |i| i+2)},
        }
    }
    lights.iter().fold(0, |acc, x| acc + usize::from(*x)).to_string()
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_1() {
        let command = "turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500";
        let result = part1(command);
        assert_eq!(result, "1001996");
    }
}
