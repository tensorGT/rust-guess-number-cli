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

