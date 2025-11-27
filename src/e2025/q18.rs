struct Plant {
    threshold: i64,
    energy: i64,
    connections: Vec<(usize, i64)>,
    done: bool,
}

impl Plant {
    fn new(input: &str) -> Plant {
        let (head, rest) = input.split_once(":\n").unwrap();
        let tokens1 = head.split_whitespace().collect::<Vec<&str>>();
        let mut plant = Plant {
            threshold: tokens1.last().unwrap().parse::<i64>().unwrap(),
            connections: vec![],
            energy: 0,
            done: false,
        };
        for line in rest.lines() {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            if tokens[1] == "free" {
                plant.done = true;
                plant.energy = plant.threshold;
            } else {
                let thickness = tokens.last().unwrap().parse::<i64>().unwrap();
                let target = tokens[4].parse::<usize>().unwrap() - 1;
                plant.connections.push((target, thickness));
            }
            // sort connections by thickness descending
            plant.connections.sort_by_key(|(_, t)| *t);
            plant.connections.reverse();
        }
        plant
    }
}

fn compute_max(plants: &mut Vec<Plant>, index: usize, positive: bool) -> i64 {
    if !plants[index].done {
        if plants[index].connections.is_empty() {
            plants[index].done = true;
            plants[index].energy = if positive { plants[index].threshold } else { 0 };
            return plants[index].energy;
        }
        let mut total = 0;
        for conn in 0..plants[index].connections.len() {
            let (target, thickness) = plants[index].connections[conn];
            let branch_positive = positive ^ (thickness < 0);
            total += compute_max(plants, target, branch_positive) * thickness;
        }
        if total >= plants[index].threshold {
            plants[index].energy = total;
        }
        plants[index].done = true;
    }
    plants[index].energy
}

pub fn part1(input: &str) -> String {
    let mut plants = input.split("\n\n").map(|block| Plant::new(block)).collect::<Vec<Plant>>();
    let last = plants.len() - 1;
    compute_max(&mut plants, last, true).to_string()
}

fn reset(plants: &mut Vec<Plant>, states: Vec<i64>) {
    // set the inputs
    for (i, state) in states.iter().enumerate() {
        plants[i].energy = *state;
        plants[i].done = true;
    }
    // reset the rest
    for i in states.len()..plants.len() {
        plants[i].done = false;
        plants[i].energy = 0;
    }
}

fn eval(plants: &mut Vec<Plant>, tests: &str) -> Vec<i64> {
    let last = plants.len() - 1;
    tests
        .lines()
        .map(|line| {
            let states = line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            reset(plants, states);
            compute_max(plants, last, true)
        })
        .collect::<Vec<i64>>()
}

pub fn part2(input: &str) -> String {
    let (input1, tests) = input.split_once("\n\n\n").unwrap();
    let mut plants = input1.split("\n\n").map(|block| Plant::new(block)).collect::<Vec<Plant>>();
    eval(&mut plants, tests).iter().sum::<i64>().to_string()
}

pub fn part3(input: &str) -> String {
    let (input1, tests) = input.split_once("\n\n\n").unwrap();
    let mut plants = input1.split("\n\n").map(|block| Plant::new(block)).collect::<Vec<Plant>>();
    let last = plants.len() - 1;
    reset(&mut plants, vec![]);
    let greedy_score = compute_max(&mut plants, last, true);
    eval(&mut plants, tests)
        .iter()
        .map(|energy| if *energy > 0 { greedy_score - *energy } else { 0 })
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "\
Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 17:
- branch to Plant 1 with thickness 15
- branch to Plant 2 with thickness 3

Plant 5 with thickness 24:
- branch to Plant 2 with thickness 11
- branch to Plant 3 with thickness 13

Plant 6 with thickness 15:
- branch to Plant 3 with thickness 14

Plant 7 with thickness 10:
- branch to Plant 4 with thickness 15
- branch to Plant 5 with thickness 21
- branch to Plant 6 with thickness 34";
        assert_eq!(part1(input), "774");
    }

    #[test]
    fn test_part2() {
        let input = "\
Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 10:
- branch to Plant 1 with thickness -25
- branch to Plant 2 with thickness 17
- branch to Plant 3 with thickness 12

Plant 5 with thickness 14:
- branch to Plant 1 with thickness 14
- branch to Plant 2 with thickness -26
- branch to Plant 3 with thickness 15

Plant 6 with thickness 150:
- branch to Plant 4 with thickness 5
- branch to Plant 5 with thickness 6


1 0 1
0 0 1
0 1 1";
        assert_eq!(part2(input), "324");
    }

    #[test]
    fn test_part3() {
        let input = "\
Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 1:
- free branch with thickness 1

Plant 5 with thickness 8:
- branch to Plant 1 with thickness -8
- branch to Plant 2 with thickness 11
- branch to Plant 3 with thickness 13
- branch to Plant 4 with thickness -7

Plant 6 with thickness 7:
- branch to Plant 1 with thickness 14
- branch to Plant 2 with thickness -9
- branch to Plant 3 with thickness 12
- branch to Plant 4 with thickness 9

Plant 7 with thickness 23:
- branch to Plant 5 with thickness 17
- branch to Plant 6 with thickness 18


0 1 0 0
0 1 0 1
0 1 1 1
1 1 0 1";
        assert_eq!(part3(input), "946");
    }
}
