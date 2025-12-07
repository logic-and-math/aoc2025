use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day_04_input").unwrap();
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let row: Vec<char> = line.unwrap().chars().collect();
        matrix.push(row);
    }

    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut res = 0;
    loop {
        let mut indexes_to_remove: Vec<(usize, usize)> = vec![];

        for (i, row) in matrix.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if *val != '@' {
                    continue;
                }

                let count: i32 = dirs
                    .iter()
                    .filter_map(|(di, dj)| {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;

                        let ni: usize = ni.try_into().ok()?;
                        let nj: usize = nj.try_into().ok()?;

                        matrix
                            .get(ni)
                            .and_then(|r| r.get(nj))
                            .map(|c| if *c == '@' { 1 } else { 0 })
                    })
                    .sum();

                if count < 4 {
                    indexes_to_remove.push((i, j));
                }
            }
        }

        if indexes_to_remove.is_empty() {
            break;
        }

        res += indexes_to_remove.len();

        for (i, j) in indexes_to_remove {
            matrix[i][j] = '.';
        }
    }

    println!("{}", res);
}
