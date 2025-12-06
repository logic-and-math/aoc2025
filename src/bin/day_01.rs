use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputs/day_01_input").unwrap();
    let reader = BufReader::new(file);

    let mut loc = 50;
    let mut res = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let op = line[0..1].to_string();
        let num: i32 = line[1..].parse().unwrap();

        // PART 1
        // if op == "L" {
        //     loc = loc - num;
        // } else {
        //     loc = loc + num;
        // }

        // if loc % 100 == 0 {
        //     res += 1;
        // }

        // PART 2
        let prev_loc = loc;

        res += num / 100;

        if op == "L" {
            loc = loc - num;
        } else {
            loc = loc + num;
        }

        loc = loc % 100;
        if loc < 0 {
            loc = 100 + loc;
        }

        if op == "L" && loc > prev_loc && prev_loc != 0 {
            res += 1;
        }

        if op == "R" && loc < prev_loc && loc != 0 {
            res += 1;
        }

        if loc % 100 == 0 {
            res += 1;
        }
    }

    println!("{}", res);
}
