use std::time::Instant;

fn read_input(event: u32, quest: u32, part: u32) -> String {
    let path = format!("input/everybody_codes_e{event}_q{quest:02}_p{part}.txt");
    std::fs::read_to_string(path).unwrap_or_default()
}

fn main() {
    let solutions = e2025();
    for sol in solutions {
        let input1 = read_input(sol.event, sol.quest, 1);
        let input2 = read_input(sol.event, sol.quest, 2);
        let input3: String = read_input(sol.event, sol.quest, 3);

        let start1 = Instant::now();
        let result1 = (sol.part1)(&input1);
        let duration1 = start1.elapsed();

        let start2 = Instant::now();
        let result2 = (sol.part2)(&input2);
        let duration2 = start2.elapsed();

        let start3 = Instant::now();
        let result3 = (sol.part3)(&input3);
        let duration3 = start3.elapsed();

        println!(
            "Event {} Quest {}: Part 1: {} ({} ms), Part 2: {} ({} ms), Part 3: {} ({} ms)",
            sol.event,
            sol.quest,
            result1,
            duration1.as_millis(),
            result2,
            duration2.as_millis(),
            result3,
            duration3.as_millis()
        ); 
    }   
}

struct Solution {
    event: u32,
    quest: u32,
    part1: fn(&str) -> String,
    part2: fn(&str) -> String,
    part3: fn(&str) -> String,
}

macro_rules! run {
    ($event:tt $($quest:tt),*) => {
        fn $event() -> Vec<Solution> {
            vec![$({
                use everybody_codes::$event::$quest::*;
                Solution {
                    event: stringify!($event)[1..].parse::<u32>().unwrap(),
                    quest: stringify!($quest)[1..].parse::<u32>().unwrap(),
                    part1: |notes: &str| part1(notes).to_string(),
                    part2: |notes: &str| part2(notes).to_string(),
                    part3: |notes: &str| part3(notes).to_string(),
                }
            },)*]
        }
    }
}

run!(e2025
    q01, q02, q03, q04
);