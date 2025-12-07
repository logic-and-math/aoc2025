use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day_07_input").unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        grid.push(line.chars().collect());
    }

    let start_index = grid[0].iter().position(|v| *v == 'S').unwrap();

    let mut res_at_split: HashMap<(usize, usize), i64> = HashMap::new();
    let res = solve(1, start_index as i64, &grid, &mut res_at_split);

    println!("P1: {:?}", res_at_split.len());
    println!("P2: {:?}", res);
}

fn solve(
    i: i64,
    j: i64,
    grid: &Vec<Vec<char>>,
    res_at_split: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    if j < 0 || j >= (grid[0].len() as i64) {
        return 1;
    }
    if i >= (grid.len() as i64) {
        return 1;
    }

    if grid[i as usize][j as usize] == '.' {
        return solve(i + 1, j, grid, res_at_split);
    }

    // split already visited so we know the result
    if let Some(&res) = res_at_split.get(&(i as usize, j as usize)) {
        return res;
    }

    let mut res = 0;
    if j > 0 {
        res += solve(i + 1, j - 1, grid, res_at_split);
    }
    if j < (grid[i as usize].len() - 1) as i64 {
        res += solve(i + 1, j + 1, grid, res_at_split);
    }

    res_at_split.insert((i as usize, j as usize), res);

    return res;
}
