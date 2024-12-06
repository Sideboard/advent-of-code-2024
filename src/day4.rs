#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let n_lines = input.lines().count();
    let line_len_list = input.lines().map(|line| line.chars().count()).collect::<Vec<_>>();
    let n_chars = line_len_list[0];
    let is_same_len = line_len_list.iter().all(|&len| len == n_chars);
    assert!(is_same_len, "All lines must have the same length");

    let mut matrix = vec![vec![' '; n_chars]; n_lines];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            matrix[i][j] = c;
        }
    }

    let mut count = 0;
    for (i, line) in matrix.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c != &'X' {
                continue;
            }
            for (di, dj) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].iter() {
                let ii = i + 3 * *di as usize;
                let jj = j + 3 * *dj as usize;
                if ii >= n_lines || ii < 0 || jj >= n_chars || jj < 0 {
                    continue;
                }
                let mut mas = String::new();
                for k in 1..4 {
                    let iii = i + k * *di as usize;
                    let jjj = j + k * *dj as usize;
                    mas.push(matrix[iii][jjj]);
                }
                if mas == "MAS" {
                    count += 1;
                }
            }
        }
    }

    return count;
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let n_lines = input.lines().count();
    let line_len_list = input.lines().map(|line| line.chars().count()).collect::<Vec<_>>();
    let n_chars = line_len_list[0];
    let is_same_len = line_len_list.iter().all(|&len| len == n_chars);
    assert!(is_same_len, "All lines must have the same length");

    let mut matrix = vec![vec![' '; n_chars]; n_lines];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            matrix[i][j] = c;
        }
    }

    let mut count = 0;
    for (i, line) in matrix.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c != &'A' {
                continue;
            }
            if i == 0 || j == 0 || i == n_lines - 1 || j == n_chars - 1 {
                continue;
            }
            let ip = i + 1;
            let jp = j + 1;
            let im = i - 1;
            let jm = j - 1;
            if (matrix[im][jm] == 'M' && matrix[ip][jp] == 'S'  ||
                matrix[im][jm] == 'S' && matrix[ip][jp] == 'M') &&
               (matrix[im][jp] == 'M' && matrix[ip][jm] == 'S' ||
                matrix[im][jp] == 'S' && matrix[ip][jm] == 'M') {
                println!("{}{}{}", matrix[im][jp], matrix[i][j], matrix[ip][jm]);
                count += 1;
            }
        }
    }

    return count;
}