#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut safe_lines = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        if is_safe(&nums) {
            safe_lines += 1;
        }
    }
    return safe_lines;
}


fn is_safe(nums: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = nums.windows(2).map(|x| x[1] - x[0]).collect();
    let zeros = diffs.iter().filter(|&x| *x == 0).count();
    let over = diffs.iter().filter(|&x| *x < -3 || *x > 3).count();
    let pos = diffs.iter().filter(|&x| *x > 0).count();
    let neg = diffs.iter().filter(|&x| *x < 0).count();

    return zeros == 0 && over == 0 && (pos == 0 || neg == 0);
}

fn is_mostly_safe(nums: &Vec<i32>, may_err: bool) -> bool {
    if is_safe(nums) {
        return true;
    }

    if !may_err {
        return false;
    }

    for i in 0..nums.len() {
        let mut new_nums = nums.clone();
        new_nums.remove(i);
        if is_mostly_safe(&new_nums, false) {
            return true;
        }
    }

    return false;
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut safe_lines = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        if is_mostly_safe(&nums, true) {
            safe_lines += 1;
        }
    }
    return safe_lines;
}
