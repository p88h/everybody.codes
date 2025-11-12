pub struct Solution {
    pub event: i32,
    pub quest: i32,
    pub part1: fn(&str) -> String,
    pub part2: fn(&str) -> String,
    pub part3: fn(&str) -> String,
}

macro_rules! library {
    ($($event:tt { $($quest:tt),* }),*) => {
        $(
            pub mod $event {
                $(pub mod $quest;)*
            }
            pub fn $event() -> Vec<Solution> {
                vec![$({
                    use $event::$quest::*;
                    Solution {
                        event: stringify!($event)[1..].parse::<i32>().unwrap(),
                        quest: stringify!($quest)[1..].parse::<i32>().unwrap(),
                        part1: |notes: &str| part1(notes).to_string(),
                        part2: |notes: &str| part2(notes).to_string(),
                        part3: |notes: &str| part3(notes).to_string(),
                    }
                },)*]
            }
        )*

        pub fn solutions() -> Vec<Solution> {
            let mut sols = Vec::new();
            $(
                sols.extend($event());
            )*
            sols
        }
    }
}

library!(
    s01 { q01, q02, q03 },
    e2025 { q01, q02, q03, q04, q05, q06, q07 }
);
