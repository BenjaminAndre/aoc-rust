use std::cmp::min;
use std::io::{self};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use aoc_client::AocClient;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;

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

fn main() {
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

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let _ = clearscreen::clear();
}
