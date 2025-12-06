use core::{num, str};
use std::{
    fs::{read, File},
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs//day_06_input").unwrap();
    let reader = BufReader::new(file);

    let mut numbers_per_equation: Vec<Vec<i64>> = vec![vec![]];
    let mut operations: Vec<String> = vec![];

    // P1
    // for line in reader.lines() {
    //     let line = line.unwrap();
    //     let parts = line
    //         .split_whitespace()
    //         .map(|s| s.to_string())
    //         .collect::<Vec<_>>();

    //     if parts[0] == "+" || parts[0] == "*" {
    //         operations.extend(parts.clone());
    //     } else {
    //         let parts: Vec<i64> = parts.iter().map(|p| p.parse::<i64>().unwrap()).collect();

    //         for (i, &p) in parts.iter().enumerate() {
    //             if let Some(vec) = numbers_per_equation.get_mut(i) {
    //                 vec.push(p);
    //             } else {
    //                 numbers_per_equation.push(vec![p]);
    //             }
    //         }
    //     }
    // }
    // P1

    // P2
    let mut grid: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        grid.push(line.chars().collect());
    }

    // lets read column by column
    for j in 0..grid[0].len() {
        let mut number_chars: Vec<char> = vec![];

        for i in 0..grid.len() {
            let value = grid[i][j];
            if value.is_ascii_digit() {
                number_chars.push(value);
            } else if value == '*' || value == '+' {
                operations.push(value.to_string());
            }
        }

        // empty column so lets move to next equation
        if number_chars.is_empty() {
            numbers_per_equation.push(vec![]);
            continue;
        }

        numbers_per_equation.last_mut().unwrap().push(
            number_chars
                .iter()
                .collect::<String>()
                .parse::<i64>()
                .unwrap(),
        );
    }
    // P2

    let mut res = 0;
    for (i, n) in numbers_per_equation.iter().enumerate() {
        let operation = &operations[i];
        let problem_res = if operation == "+" {
            n.iter().sum::<i64>()
        } else {
            n.iter().fold(1_i64, |a, &b| a * b)
        };

        res += problem_res;
    }

    println!("{}", res);
}
