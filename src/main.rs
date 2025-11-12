use std::time::Instant;

fn read_input(event: i32, quest: i32, part: i32) -> String {
    let path = format!("input/everybody_codes_e{event}_q{quest:02}_p{part}.txt");
    std::fs::read_to_string(path).unwrap_or_default()
}

fn maybe_parse_arg(args: &Vec<String>, pos: usize, default: i32) -> i32 {
    if args.len() <= pos {
        return default;
    }
    if args[pos].is_empty() || args[pos] == "all" {
        return -1;
    }
    return args[pos].parse().unwrap_or(-1);
}

fn run_part(event: i32, quest: i32, part: i32, func: fn(&str) -> String) -> (String, String) {
    let input = read_input(event, quest, part);
    let start = Instant::now();
    let result = func(&input);
    for _ in 0..9 {
        let other = func(&input);
        if other != result {
            panic!(
                "Inconsistent results for event {}, quest {}, part {}: '{}' vs '{}'",
                event, quest, part, result, other
            );
        }
    }
    let duration = start.elapsed() / 10;
    // auto format duration
    if duration.as_nanos() < 1_000 {
        (result, format!("({} ns)", duration.as_nanos()))
    } else if duration.as_micros() < 1_000 {
        (result, format!("({} µs)", duration.as_micros()))
    } else if duration.as_millis() < 1_000 {
        (result, format!("({} ms)", duration.as_millis()))
    } else {
        (result, format!("({} s)", duration.as_secs_f64()))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let solutions = everybody_codes::solutions();
    let event = maybe_parse_arg(&args, 1, solutions[solutions.len() - 1].event);
    let quest = maybe_parse_arg(&args, 2, -1);
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
        println!("{label} Part 1: {r1:16} {d1:8}  Part 2: {r2:16} {d2:8}  Part 3: {r3:16} {d3:8}");
    }
}
