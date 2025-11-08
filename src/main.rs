use std::time::Instant;

fn read_input(event: u32, quest: u32, part: u32) -> String {
    let path = format!("input/everybody_codes_e{event}_q{quest:02}_p{part}.txt");
    std::fs::read_to_string(path).unwrap_or_default()
}

fn main() {
    let solutions = everybody_codes::e2025();
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

