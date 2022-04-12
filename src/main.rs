use std::fs;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const SCORE_FILE: &str = "best_score.txt";

fn simple_random(max_val: u32) -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(42);
    (nanos % max_val) + 1
}

fn load_best() -> Option<u32> {
    fs::read_to_string(SCORE_FILE)
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
}

fn save_best(score: u32) {
    let _ = fs::write(SCORE_FILE, score.to_string());
}

fn main() {
    let secret = simple_random(100);
    let best = load_best();
    println!("guess the number between 1 and 100!");
    if let Some(b) = best {
        println!("current best score: {} tries", b);
    }

    let mut tries: u32 = 0;

    loop {
        print!("your guess: ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("could not read input");
            continue;
        }
