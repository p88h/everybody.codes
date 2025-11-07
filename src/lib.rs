macro_rules! library {
    ($year:tt $($day:tt),*) => {
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

library!(e2025
    q01, q02, q03, q04
);
