
#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut safe_lines = 0;
    'lines: for line in input.lines() {
        let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let diffs: Vec<i32> = nums.windows(2).map(|x| x[1] - x[0]).collect();
        let inc = diffs[0] > 0;

        for diff in diffs {
            if diff == 0 || diff < -3 || diff > 3 || diff > 0 && !inc || diff < 0 && inc {
                continue 'lines;
            }
        }
        safe_lines += 1;
    }
    return safe_lines;
}
