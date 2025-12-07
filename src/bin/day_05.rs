use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day_05_input").unwrap();
    let reader = BufReader::new(file);

    let mut fresh_ranges: Vec<(i64, i64)> = vec![];
    let mut ingredients: Vec<i64> = vec![];

    let mut finished_ranges = false;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            finished_ranges = true;
            continue;
        }

        if finished_ranges {
            ingredients.push(line.parse::<i64>().unwrap());
        } else {
            let parts: Vec<&str> = line.split("-").collect();
            fresh_ranges.push((
                parts[0].parse::<i64>().unwrap(),
                parts[1].parse::<i64>().unwrap(),
            ));
        }
    }

    //P1
    let mut res = 0;
    for i in ingredients {
        for (start, end) in fresh_ranges.iter() {
            if i >= *start && i <= *end {
                res += 1;
                break;
            }
        }
    }

    println!("{}", res);

    //P2
    fresh_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged_ranges: Vec<(i64, i64)> = vec![fresh_ranges[0]];

    for range in fresh_ranges.iter().skip(1) {
        let last_merged = merged_ranges.last_mut().unwrap();

        if last_merged.1 >= range.0 {
            // if fully contained dont decrease the end of the range
            if last_merged.1 < range.1 {
                last_merged.1 = range.1;
            }
        } else {
            merged_ranges.push(*range);
        }
    }

    res = merged_ranges.iter().map(|r| r.1 - r.0 + 1).sum::<i64>();

    println!("{:?}", res);
}
