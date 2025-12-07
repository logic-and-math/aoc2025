use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day_03_input").unwrap();
    let reader = BufReader::new(file);

    let res: u64 = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        })
        //P1
        // .map(|digits| {
        //     let mut max_first = MIN;
        //     let mut max_second = MIN;
        //     for w in digits.windows(2) {
        //         if w[0] > max_first {
        //             max_first = w[0];
        //             max_second = w[1];
        //         } else {
        //             max_second = max_second.max(w[0]).max(w[1]);
        //         }
        //     }
        //     return max_first * 10 + max_second;
        // })
        //P2
        .map(|digits| {
            let mut max_digits: [u64; 12] = [u64::MIN; 12];
            let mut last_digit_pos = 0;
            max_digits[0] = digits[0];

            for i in 0..12 {
                for pos in (last_digit_pos + 1)..=(digits.len() - 12 + i) {
                    if digits[pos] > max_digits[i] {
                        max_digits[i] = digits[pos];
                        last_digit_pos = pos;
                    }
                }
            }

            max_digits
                .iter()
                .enumerate()
                .map(|x| x.1 * 10_u64.pow(12 - (x.0 as u32) - 1))
                .sum::<u64>()
        })
        .sum();

    println!("{}", res);
}
