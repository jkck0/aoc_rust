pub mod util {
    pub mod disjoint_set;
    pub mod grid;
    pub mod point;
}

macro_rules! year {
    ($year:tt $($day:tt),*) => {
        pub mod $year {
            $(pub mod $day;)*
        }
    };
}

year!(
    year2015
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10
);

year!(
    year2025
    day01, day02, day03, day04, day05, day06, day07, day08
);
