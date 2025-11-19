use std::collections::HashSet;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

struct Dice {
    faces: Vec<i64>,
    seed: i64,
    pulse: i64,
    counter: i64,
    prev: usize,
}

impl Dice {
    fn new(input: &str) -> Self {
        let mut parts = input.trim().split_whitespace();
        parts.next(); // ignore index
        let faces_part = parts.next().unwrap();
        let seed_part = parts.next().unwrap();
        let faces_str = faces_part.strip_prefix("faces=[").unwrap().strip_suffix("]").unwrap();
        let faces = faces_str.split(',').filter_map(|s| s.parse::<i64>().ok()).collect::<Vec<i64>>();
        let seed = seed_part.strip_prefix("seed=").unwrap().parse::<i64>().unwrap();
        Dice { faces, seed, pulse: seed, counter: 1, prev: 0 }
    }

    fn spin(&mut self) -> i64 {
        let idx = self.counter * self.pulse;
        self.pulse += idx;
        self.pulse %= self.seed;
        self.pulse += 1 + self.counter + self.seed;
        self.counter += 1;
        self.prev = (idx as usize + self.prev) % self.faces.len();
        self.faces[self.prev]
    }
}

pub fn part1(input: &str) -> String {
    let mut dice_set = input.lines().map(|line| Dice::new(line)).collect::<Vec<Dice>>();
    let mut score = 0;
    let mut round = 0;
    while score < 10000 {
        round += 1;
        for dice in dice_set.iter_mut() {
            let roll = dice.spin();
            score += roll;
        }
    }
    round.to_string()
}

pub fn part2(input: &str) -> String {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut dice_set = first.lines().map(|line| Dice::new(line)).collect::<Vec<Dice>>();
    let digits = second.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
    let mut results: Vec<(i64, usize)> = vec![];
    for (id, dice) in dice_set.iter_mut().enumerate() {
        for &digit in digits.iter() {
            while dice.spin() as usize != digit {}
        }
        results.push((dice.counter, id));
    }
    results.sort();
    results.iter().map(|(_, id)| (id + 1).to_string()).collect::<Vec<String>>().join(",")
}

fn play_dice(dice: &mut Dice, grid: &Vec<Vec<i64>>) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut spin = dice.spin();
    let mut current: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if spin == grid[r][c] {
                current.insert((r, c));
            }
        }
    }
    while current.len() > 0 {
        let mut next: HashSet<(usize, usize)> = HashSet::new();
        spin = dice.spin();
        for &(r, c) in current.iter() {
            visited.insert((r, c));
            if spin == grid[r][c] {
                next.insert((r, c));
            }
            if r > 0 && spin == grid[r - 1][c] {
                next.insert((r - 1, c));
            }
            if r + 1 < grid.len() && spin == grid[r + 1][c] {
                next.insert((r + 1, c));
            }
            if c > 0 && spin == grid[r][c - 1] {
                next.insert((r, c - 1));
            }
            if c + 1 < grid[0].len() && spin == grid[r][c + 1] {
                next.insert((r, c + 1));
            }
        }
        current = next;
    }
    visited
}

pub fn part3(input: &str) -> String {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut dice_set = first.lines().map(|line| Dice::new(line)).collect::<Vec<Dice>>();
    let grid = second
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i64).collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>();
    let all_visited =
        dice_set.par_iter_mut().map(|dice| play_dice(dice, &grid)).reduce(HashSet::new, |mut acc, visited| {
            acc.extend(visited);
            acc
        });
    all_visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "\
1: faces=[1,2,3,4,5,6] seed=7
2: faces=[-1,1,-1,1,-1] seed=13
3: faces=[9,8,7,8,9] seed=17";
        assert_eq!(part1(input), "844");
    }

    #[test]
    fn test_part2() {
        let input = "1: faces=[1,2,3,4,5,6,7,8,9] seed=13
2: faces=[1,2,3,4,5,6,7,8,9] seed=29
3: faces=[1,2,3,4,5,6,7,8,9] seed=37
4: faces=[1,2,3,4,5,6,7,8,9] seed=43

51257284";
        assert_eq!(part2(input), "1,3,4,2");
    }

    #[test]
    fn test_part3() {
        let input = "1: faces=[1,2,3,4,5,6,7,8,9] seed=13

1523758297
4822941583
7627997892
4397697132
1799773472";
        assert_eq!(part3(input), "33");
    }

    #[test]
    fn test_part3_large() {
        let input = "\
1: faces=[1,2,3,4,5,6,7,8,9] seed=339211
2: faces=[1,2,3,4,5,6,7,8,9] seed=339517
3: faces=[1,2,3,4,5,6,7,8,9] seed=339769
4: faces=[1,2,3,4,5,6,7,8,9] seed=339049
5: faces=[1,2,3,4,5,6,7,8,9] seed=338959
6: faces=[1,2,3,4,5,6,7,8,9] seed=340111
7: faces=[1,2,3,4,5,6,7,8,9] seed=339679
8: faces=[1,2,3,4,5,6,7,8,9] seed=339121
9: faces=[1,2,3,4,5,6,7,8,9] seed=338851

94129478611916584144567479397512595367821487689499329543245932151
45326719759656232865938673559697851227323497148536117267854241288
44425936468288462848395149959678842215853561564389485413422813386
64558359733811767982282485122488769592428259771817485135798694145
17145764554656647599363636643624443394141749674594439266267914738
89687344812176758317288229174788352467288242171125512646356965953
72436836424726621961424876248346712363842529736689287535527512173
18295771348356417112646514812963612341591986162693455745689374361
56445661964557624561727322332461348422854112571195242864151143533
77537797151985578367895335725777225518396231453691496787716283477
37666899356978497489345173784484282858559847597424967325966961183
26423131974661694562195955939964966722352323745667498767153191712
99821139398463125478734415536932821142852955688669975837535594682
17768265895455681847771319336534851247125295119363323122744953158
25655579913247189643736314385964221584784477663153155222414634387
62881693835262899543396571369125158422922821541597516885389448546
71751114798332662666694134456689735288947441583123159231519473489
94932859392146885633942828174712588132581248183339538341386944937
53828883514868969493559487848248847169557825166338328352792866332
54329673374115668178556175692459528276819221245996289611868492731
97799599164121988455613343238811122469229423272696867686953891233
56249752581283778997317243845187615584225693829653495119532543712
39171354221177772498317826968247939792845866251456175433557619425
56425749216121421458547849142439211299266255482219915528173596421
48679971256541851497913572722857258171788611888347747362797259539
32676924489943265499379145361515824954991343541956993467914114579
45733396847369746189956225365375253819969643711633873473662833395
42291594527499443926636288241672629499242134451937866578992236427
47615394883193571183931424851238451485822477158595936634849167455
16742896921499963113544858716552428241241973653655714294517865841
57496921774277833341488566199458567884285639693339942468585269698
22734249697451127789698862596688824444191118289959746248348491792
28575193613471799766369217455617858422158428235521423695479745656
74234343226976999161289522983885254212712515669681365845434541257
43457237419516813368452247532764649744546181229533942414983335895";
        assert_eq!(part3(input), "1125");
    }
}
