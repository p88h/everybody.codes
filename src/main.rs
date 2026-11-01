mod download;

use std::path::Path;
use std::time::{Duration, Instant};

fn read_input(event: i32, quest: i32, part: i32) -> String {
    let path = format!("input/everybody_codes_e{event}_q{quest:02}_p{part}.txt");
    if let Ok(notes) = std::fs::read_to_string(&path) {
        return notes;
    }
    match download::download_to(event, quest, part, Path::new(&path)) {
        Ok(notes) => {
            eprintln!("Downloaded {path}");
            notes
        }
        Err(err) => {
            eprintln!("Missing {path}, download failed: {err}");
            String::new()
        }
    }
}

fn maybe_parse_arg(args: &Vec<String>, pos: usize, default: i32) -> i32 {
    if args.len() <= pos {
        return default;
    }
    if args[pos].is_empty() || args[pos] == "all" {
        return -1;
    }
    if let Some(first_char) = args[pos].chars().next() {
        if first_char.is_alphabetic() {
            return args[pos][1..].parse().unwrap_or(-1);
        }
    }
    return args[pos].parse().unwrap_or(-1);
}

fn run_part(event: i32, quest: i32, part: i32, func: fn(&str) -> String) -> (String, Duration) {
    let input = read_input(event, quest, part);
    let start = Instant::now();
    let result = func(&input);
    let mut loops = 10;
    let mut done_loops = 1;
    loop {
        for _ in 0..loops {
            let other = func(&input);
            if other != result {
                panic!(
                    "Inconsistent results for event {}, quest {}, part {}: '{}' vs '{}'",
                    event, quest, part, result, other
                );
            }
        }
        done_loops += loops;
        if start.elapsed() >= Duration::from_millis(100) {
            break;
        }
        if start.elapsed() < Duration::from_millis(1) {
            loops *= 10;
        }
    } 
    // println!("Event {} Quest {} Part {} Loops: {}", event, quest, part, done_loops);
    let duration = start.elapsed() / done_loops;
    (result, duration)
}

fn fmt_duration(duration: Duration) -> String {
    // 4 significant characters; switch to the larger unit once the value reaches 0.10
    let nanos = duration.as_nanos() as f64;
    let units = [("ns", 1.0), ("µs", 1e3), ("ms", 1e6), ("s", 1e9)];
    let (unit, scale) = units.iter().rev().find(|(_, s)| nanos / s >= 0.1).copied().unwrap_or(units[0]);
    let v = nanos / scale;
    if v < 9.995 {
        format!("{v:.2} {unit}")
    } else if v < 99.95 {
        format!("{v:.1} {unit}")
    } else {
        format!("{v:.0} {unit}")
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let solutions = everybody_codes::solutions();
    let event = maybe_parse_arg(&args, 1, solutions[solutions.len() - 1].event);
    let quest = maybe_parse_arg(&args, 2, -1);
    let mut total = Duration::new(0, 0);
    for sol in solutions {
        if (event != -1 && sol.event != event) || (quest != -1 && sol.quest != quest) {
            continue;
        }
        let (r1, d1) = run_part(sol.event, sol.quest, 1, sol.part1);
        let (r2, d2) = run_part(sol.event, sol.quest, 2, sol.part2);
        let (r3, d3) = run_part(sol.event, sol.quest, 3, sol.part3);
        let label = if sol.event >= 1000 {
            format!("Event {} Quest {:02}", sol.event, sol.quest)
        } else {
            format!("Story {:04} Quest {:02}", sol.event, sol.quest)
        };
        println!("{label}: {} / {} / {}", fmt_duration(d1), fmt_duration(d2), fmt_duration(d3));
        println!("Part 1: {r1:16} Part 2: {r2:16} Part 3: {r3:16}");
        total += d1 + d2 + d3;
    }
    println!("Total: {}", fmt_duration(total));
}
