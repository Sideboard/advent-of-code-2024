use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let sum = re.captures_iter(input).map(|cap| {
        return &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap();
    }).sum();
    return sum;
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r"(?:(?:mul\((\d{1,3}),(\d{1,3})\))|(?:do(?:n't)?))").unwrap();
    let mut active = true;
    let mut sum = 0;
    for cap in re.captures_iter(input) {
        if &cap[0] == "do" || &cap[0] == "don't" {
            active = &cap[0] == "do";
            continue;
        }
        if active {
            sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
        }
   }
    return sum;
}
