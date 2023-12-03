use std::time::{Duration, Instant};

pub mod util;

#[derive(Debug, Clone, Copy)]
pub struct DayName {
    pub name: &'static str,
    pub day: u8,
}

#[derive(Debug)]
pub struct RunResult {
    pub name: DayName,
    pub parse_time: Duration,
    pub p1_time: Duration,
    pub p2_time: Duration,
    pub p1_result: String,
    pub p2_result: String,
}

impl RunResult {
    pub fn total_time(&self) -> Duration {
        self.parse_time + self.p1_time + self.p2_time
    }
}

pub struct Day<ParsedInput, P1Input, P1Result, P2Input, P2Result>
where
    ParsedInput: AsRef<P1Input> + AsRef<P2Input>,
    P1Input: ?Sized,
    P1Result: std::fmt::Display,
    P2Input: ?Sized,
    P2Result: std::fmt::Display,
{
    name: DayName,
    parse: Box<dyn Fn(&str) -> ParsedInput>,
    part_1: Box<dyn Fn(&P1Input) -> P1Result>,
    part_2: Box<dyn Fn(&P2Input) -> P2Result>,
}

pub trait ErasedDay {
    fn name(&self) -> DayName;
    fn run(&self, input: &str) -> RunResult;
}

impl<ParsedInput, P1Input, P1Result, P2Input, P2Result> ErasedDay
    for Day<ParsedInput, P1Input, P1Result, P2Input, P2Result>
where
    ParsedInput: AsRef<P1Input> + AsRef<P2Input>,
    P1Input: ?Sized,
    P1Result: std::fmt::Display,
    P2Input: ?Sized,
    P2Result: std::fmt::Display,
{
    fn name(&self) -> DayName {
        self.name
    }

    fn run(&self, input: &str) -> RunResult {
        let sw = Instant::now();
        let parsed_input = (self.parse)(input);
        let parse_time = sw.elapsed();

        let sw = Instant::now();
        let p1_result = (self.part_1)(parsed_input.as_ref());
        let p1_time = sw.elapsed();
        let p1_result = format!("{}", p1_result);

        let sw = Instant::now();
        let p2_result = (self.part_2)(parsed_input.as_ref());
        let p2_time = sw.elapsed();
        let p2_result = format!("{}", p2_result);

        RunResult {
            name: self.name.clone(),
            parse_time,
            p1_time,
            p2_time,
            p1_result,
            p2_result,
        }
    }
}

pub fn get_input(input_root: &std::path::Path, day_name: DayName) -> anyhow::Result<String> {
    let file_name = format!("input_{}.txt", day_name.day);
    let mut path = input_root.to_path_buf();
    path.push(file_name);

    let input = if path.exists() {
        std::fs::read_to_string(path)?
    } else {
        println!("Fetching input for day {}", day_name.day);

        let url = format!("https://adventofcode.com/2023/day/{}/input", day_name.day);
        let session_cookie = std::env::var("AOC_SESSION_COOKIE")
            .expect("Input not cached, and AOC_SESSION_COOKIE not set");

        let jar = reqwest::cookie::Jar::default();
        jar.add_cookie_str(
            &format!("session={session_cookie}"),
            &"https://adventofcode.com".parse().unwrap(),
        );
        let client = reqwest::blocking::ClientBuilder::default()
            .cookie_provider(std::sync::Arc::new(jar))
            .build()?;

        let input = client.get(url).send()?.text()?;

        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(path, &input)?;

        input
    };

    Ok(input)
}

pub fn print_results_table(results: &[RunResult]) {
    if results.len() == 0 {
        return;
    }

    fn col_width(results: &[RunResult], title: &str, len: impl Fn(&RunResult) -> usize) -> usize {
        std::cmp::max(title.len(), results.iter().map(len).max().unwrap())
    }

    let name_width = col_width(results, "Name", |r| r.name.name.len());
    let p1_result_width = col_width(results, "P1 result", |r| r.p1_result.len());
    let p2_result_width = col_width(results, "P2 result", |r| r.p2_result.len());
    let parse_time_width = col_width(results, "Parse time", |r| {
        format!("{:?}", r.parse_time).len()
    });
    let p1_time_width = col_width(results, "P1 time", |r| format!("{:?}", r.p1_time).len());
    let p2_time_width = col_width(results, "P2 time", |r| format!("{:?}", r.p2_time).len());

    let total_parse_time: Duration = results.iter().map(|r| r.parse_time).sum();
    let total_p1_time: Duration = results.iter().map(|r| r.p1_time).sum();
    let total_p2_time: Duration = results.iter().map(|r| r.p2_time).sum();
    let total_time: Duration = results.iter().map(|r| r.total_time()).sum();

    let parse_time_width = std::cmp::max(parse_time_width, format!("{:?}", total_parse_time).len());
    let p1_time_width = std::cmp::max(p1_time_width, format!("{:?}", total_p1_time).len());
    let p2_time_width = std::cmp::max(p2_time_width, format!("{:?}", total_p2_time).len());

    let total_offset = 5 // "day |"
        + name_width + 3
        + p1_result_width + 3
        + p2_result_width + 1;

    let header = format!(
        "Day | {:name_width$} | {:p1_result_width$} | {:p2_result_width$} | {:parse_time_width$} | {:p1_time_width$} | {:p2_time_width$} | Total time",
        "Name",
        "P1 result",
        "P2 result",
        "Parse time",
        "P1 time",
        "P2 time",
        name_width = name_width,
        p1_result_width = p1_result_width,
        p2_result_width = p2_result_width,
        parse_time_width = parse_time_width,
        p1_time_width = p1_time_width,
        p2_time_width = p2_time_width,
    );

    println!("{}", header);
    for _ in 0..header.len() {
        print!("-");
    }
    println!();

    let print = |result: &RunResult| {
        println!("{:02}  | {:name_width$} | {:p1_result_width$} | {:p2_result_width$} | {:parse_time_width$?} | {:p1_time_width$?} | {:p2_time_width$?} | {:?}",
            result.name.day,
            result.name.name,
            result.p1_result,
            result.p2_result,
            result.parse_time,
            result.p1_time,
            result.p2_time,
            result.total_time(),
        )
    };

    for result in results {
        print(result);
    }

    for _ in 0..header.len() {
        print!("-");
    }
    println!();

    println!(
        "{:>total_offset$} | {:parse_time_width$?} | {:p1_time_width$?} | {:p2_time_width$?} | {:?}",
        "Total",
        total_parse_time,
        total_p1_time,
        total_p2_time,
        total_time,
    );
}

macro_rules! define_days {
    ($(($name:literal, $day_num:literal, $mod:ident)),* $(,)?) => {
        $(
            mod $mod;
        )*

        pub fn all_days() -> Vec<Box<dyn ErasedDay>> {
            vec![$(
                Box::new(Day {
                    name: DayName { name: $name, day: $day_num },
                    parse: Box::new($mod::parse),
                    part_1: Box::new($mod::solve_part_1),
                    part_2: Box::new($mod::solve_part_2),
                })
            ),*]
        }
    }
}

define_days! {
    ("Trebuchet?!", 1, day_1),
    ("Cube Conundrum", 2, day_2),
    ("Gear ratios", 3, day_3),
}
