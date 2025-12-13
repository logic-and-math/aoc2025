use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day_12_input").unwrap();
    let reader = BufReader::new(file);

    let mut grids: Vec<Vec<Vec<char>>> = vec![];

    let mut sizes = Vec::<(u32, u32)>::new();
    let mut expected_presents = Vec::<Vec<u32>>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }

        if line.contains("x") {
            let (sizes_part, presents_part) = line.split_once(':').unwrap();

            let mut it = sizes_part.split('x').map(|s| s.parse().unwrap());
            sizes.push((it.next().unwrap(), it.next().unwrap()));

            expected_presents.push(
                presents_part
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            );
        } else if line.contains(":") {
            grids.push(vec![]);
        } else {
            grids.last_mut().unwrap().push(line.chars().collect());
        }
    }

    let mut can_fit = 0;
    for (i, size) in sizes.iter().enumerate() {
        let mut needed_area: usize = 0;
        for (j, n) in expected_presents[i].iter().enumerate() {
            let grid = &grids[j];
            needed_area += (*n as usize) * grid.len() * grid[0].len();
        }

        if needed_area <= (size.0 * size.1) as usize {
            can_fit += 1;
        }
    }

    println!("{:?}", can_fit);
}
