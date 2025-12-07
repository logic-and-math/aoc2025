use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day_02_input").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let pairs: Vec<(i64, i64)> = line
        .split(',')
        .map(|pair| {
            let mut parts = pair.split('-');
            (
                parts.next().unwrap().parse::<i64>().unwrap(),
                parts.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect();

    let res: i64 = pairs
        .iter()
        .map(|p| {
            (p.0..=p.1)
                .map(|x| x.to_string())
                // P1
                // .filter(|x| x.len() % 2 == 0)
                // .filter(|x| x[0..x.len() / 2] == x[x.len() / 2..])
                // P1 END
                // P2
                .filter(|x| {
                    (1..=x.len() / 2).any(|s| {
                        let chunks: Vec<&[u8]> = x.as_bytes().chunks(s).collect();
                        return chunks.iter().all(|c| c == &chunks[0]);
                    })
                })
                //P2 END
                .map(|x| x.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .sum();

    println!("{:?}", res);
}
