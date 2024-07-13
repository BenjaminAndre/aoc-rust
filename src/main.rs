use std::cmp::min;
use std::io::{self};
use std::sync::Arc;
use std::thread;
use std::env;
use std::time::{Duration, Instant};

use aoc_client::AocClient;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;


use std::{fs, process::Command, error::Error};
use itertools::Itertools;



struct AocDay<'a> {
    year: i32,
    day: u32,
    part_one: &'a Option<(i64, Box<dyn Fn(&str) -> String>)>,
    part_two: &'a Option<(i64, Box<dyn Fn(&str) -> String>)>,
}

trait Tryable {
    fn try_part(&self, part: &Option<(i64, Box<dyn Fn(&str) -> String>)>) -> Result<u64, String>;
}

impl Tryable for AocClient {
    fn try_part(&self, part: &Option<(i64, Box<dyn Fn(&str) -> String>)>) -> Result<u64, String> {
        let real_part = match part {
            Some(p) => p,
            None => return Err(String::from("Part not available")),
        };
        let _input = match self.get_input() {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };
        let func = &real_part.1;
        let now = Instant::now();
        let solution = func(&_input);
        let time: u64 = match now.elapsed().as_micros().try_into() {
            Ok(n) => n,
            Err(s) => return Err(s.to_string()),
        };
        let answer = match self.submit_answer(real_part.0, solution) {
            Ok(so) => so,
            Err(aoc_error) => return Err(aoc_error.to_string()),
        };
        match answer {
            aoc_client::SubmissionOutcome::Correct => return Ok(time),
            _ => return Err(String::from("Incorrect and can't print much more since SubmissionOutcome doesn't implement Debug")),
        };
    }
}

trait Runnable {
    fn run(&self) -> Vec<u64>;
}

impl Runnable for AocDay<'_> {
    fn run(&self) -> Vec<u64> {
        let mut solutions: Vec<u64> = Vec::new();
        let mut binding = AocClient::builder();
        let client = match binding.session_cookie_from_default_locations() {
            Ok(c) => c,
            _ => return solutions,
        };
        let client = match client.year(self.year) {
            Ok(c) => c,
            _ => return solutions,
        };
        let client = match client.day(self.day) {
            Ok(c) => c,
            _ => return solutions,
        };
        let client = match client.build() {
            Ok(c) => c,
            _ => return solutions,
        };
        match client.try_part(self.part_one) {
            Ok(t) => solutions.push(t),
            _ => return solutions,
        };
        match client.try_part(self.part_two) {
            Ok(t) => solutions.push(t),
            _ => return solutions,
        };
        solutions
    }
}

fn run_all() {
    let mut downloaded = 0;
    let total_size = 50;
    let starting_year = 2015;
    let years = 9;
    let mpb : MultiProgress = MultiProgress::new();
    let mut pbs: Vec<Arc<ProgressBar>> = vec![];

    for i in 0..years {
        let pb = mpb.add(ProgressBar::new(total_size));
        pb.set_style(ProgressStyle::with_template(&format!("{{spinner:.green}} [{{elapsed_precise}}] {} [{{bar:50.cyan/blue}}] {{pos}}★/{{len}}★ ", starting_year+i)[..]).unwrap().progress_chars("★>."));
        pbs.push(Arc::new(pb));
    }

    let mut threads = vec![];

    for i in 0..years {
        let pb = pbs[i].clone();
        threads.push(thread::spawn(move || {
            let max_stars = rand::thread_rng().gen_range(0..total_size);
            while downloaded < max_stars {
                let pos = pb.position();
                let new = min(pos + 1, total_size);
                downloaded = new;
                pb.set_position(new);
                let delay = rand::thread_rng().gen_range(80..360);
                thread::sleep(Duration::from_millis(delay));
            }
        }))
    }

    for thread in threads {
        let _ = thread.join();
    }

    let total_stars: u64 = pbs.iter().map(|apb| apb.position()).sum();
    println!("Congratulations on getting all those {}★ !", total_stars);

}

fn run_years(lowerbound : u32, higherbound: u32) {
    let mut total_stars = 0;
    let total_size = 50;
    let years = higherbound - lowerbound;
    let mpb : MultiProgress = MultiProgress::new();
    let mut pbs: Vec<Arc<ProgressBar>> = vec![];

    for i in 0..=years {
        let pb = mpb.add(ProgressBar::new(total_size));
        pb.set_style(ProgressStyle::with_template(&format!("{{spinner:.green}} [{{elapsed_precise}}] {} [{{bar:50.cyan/blue}}] {{pos}}★/{{len}}★ ", lowerbound+i)[..]).unwrap().progress_chars("★>."));
        pbs.push(Arc::new(pb));
    }

    let mut threads = vec![];

    for i in 0..=years {
        let pb = pbs[usize::try_from(i).unwrap()].clone();
        threads.push(thread::spawn(move || {
            let max_stars = rand::thread_rng().gen_range(0..total_size);
            while total_stars < max_stars {
                let pos = pb.position();
                let new = min(pos + 1, total_size);
                total_stars = new;
                pb.set_position(new);
                let delay = rand::thread_rng().gen_range(80..360);
                thread::sleep(Duration::from_millis(delay));
            }
        }))
    }

    for thread in threads {
        let _ = thread.join();
    }

    let total_stars: u64 = pbs.iter().map(|apb| apb.position()).sum();
    println!("Congratulations on getting all those {}★ !", total_stars);
}

fn run_day(year : u32, day : u32) {

}

//fn main() {
//    let args: Vec<String> = env::args().collect();
//    let arg1 = args.get(1);
//    let arg2 = args.get(2);
//    let placeholder = "".to_string();
//
//    match arg1.unwrap_or(&placeholder).parse::<u32>() {
//        Ok(year) => {
//            match arg2.unwrap_or(&placeholder).parse::<u32>() {
//                Ok(day) => run_day(year, day),
//                _ => run_years(year, year),
//            };
//        }
//        _ => run_years(2015, 2023),
//    };
//
//    let mut input = String::new();
//    let _ = io::stdin().read_line(&mut input);
//    let _ = clearscreen::clear();
//}

fn extract_microseconds(output: &str) -> Result<usize, Box<dyn Error>> {
  let out = output.lines().last().unwrap();
  let time = if out.ends_with("ms") {
    out["Time: ".len()..out.len()-2].parse::<usize>()? * 1000
  } else {
    out["Time: ".len()..out.len()-3].parse::<usize>()?
  };
  Ok(time)
}

fn main() -> Result<(), Box<dyn Error>> {
  let days = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/"))?
    .filter_map(|p| p.ok()?.path().file_stem()?.to_str().map(str::to_string))
    .sorted()
    .collect::<Vec<_>>();
  let mut total_time = 0;
  for day in &days {
    let cmd = Command::new("cargo").args(["run", "--release", "--bin", day]).output()?;
    let output = String::from_utf8(cmd.stdout)?;
    println!("Day {}:\n{}", day, output);
    total_time += extract_microseconds(&output)?;
  }
  println!("Total time: {}ms", total_time / 1000);
  Ok(())
}
