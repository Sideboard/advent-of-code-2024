use std::collections::HashMap;

type TwoLists = (Vec<u32>, Vec<u32>);

fn count(list: &[u32]) -> HashMap<&u32, u32> {
    let mut counter = HashMap::new();
    for item in list {
        if counter.get(item) == None {
            counter.insert(item, 1_u32);
        } else {
            counter.insert(item, counter.get(item).unwrap() + 1_u32);
        }
    }
    return counter;
}

fn parse_input(input: &str) -> TwoLists {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines() {
        let mut ints = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
        list1.push(ints.next().unwrap());
        list2.push(ints.next().unwrap());
    }
    return (list1, list2);
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let lists = parse_input(input);
    let mut list1 = lists.0.clone();
    let mut list2 = lists.1.clone();
    list1.sort();
    list2.sort();

    let sum = list1.iter().zip(list2.iter()).map(|(a, b)| a.abs_diff(*b)).sum();
    return sum;
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let lists = parse_input(input);
    let counter1 = count(&lists.0);
    let counter2 = count(&lists.1);
    let mut sum = 0;
    for (k, v) in counter1 {
        let count2 = counter2.get(k).unwrap_or(&0);
        sum += k * v * count2;
    }
    return sum;
}
