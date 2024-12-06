use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let sum = re.captures_iter(input).map(|cap| {
        return &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap();
    }).sum();
    return sum;
}
