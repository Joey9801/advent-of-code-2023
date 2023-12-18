#[derive(Debug)]
pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn ways_to_win(self) -> u64 {
        // The numbers are small enough that we should be able to get away with
        // floating point math

        // time = hold + distance/hold
        // => beats the record when:
        //  hold + distance / hold - record < 0
        //  hold^2 - hold * record + distance < 0
        //
        // Has roots: hold = (record +- sqrt(record^2 - 4 * distance)) / 2

        let record = self.time as f64;
        let distance = self.distance as f64;

        let s = (record * record - 4.0 * distance).sqrt();
        let r1 = (record - s) / 2.0;
        let r2 = (record + s) / 2.0;

        let r1 = r1.ceil() as u64;
        let r2 = r2.floor() as u64;

        r2 - r1 + 1
    }
}

pub fn parse(input: &str) -> String {
    // String like:
    // Time:      7  15   30
    // Distance:  9  40  200
    input.to_string()
}

pub fn solve_part_1(input: &str) -> u64 {
    let (first_line, second_line) = input.split_once("\n").unwrap();

    let times = first_line
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap());
    let distances = second_line
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap());

    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .map(Race::ways_to_win)
        .product()
}

pub fn solve_part_2(input: &str) -> u64 {
    // Merge the races - treat all the digits of time/distance as single large
    // numbers

    let (first_line, second_line) = input.split_once("\n").unwrap();
    let time = first_line
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    let distance = second_line
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    Race { time, distance }.ways_to_win()
}
