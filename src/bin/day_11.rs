use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day_11_input").unwrap();
    let reader = BufReader::new(file);

    let mut inputs_to_outputs: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(':').collect();

        let key = parts[0].trim().to_string();
        let values: Vec<String> = parts[1]
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        inputs_to_outputs.insert(key, values);
    }

    let inputs_to_outputs = inputs_to_outputs;

    let mut cache: HashMap<(String, bool, bool), i64> = HashMap::new();

    let res = solve(
        &("svr".to_string()),
        &inputs_to_outputs,
        &mut cache,
        false,
        false,
    );

    println!("{}", res);
}

fn solve(
    input: &String,
    inputs_to_outputs: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<(String, bool, bool), i64>,
    fft_found: bool,
    dac_found: bool,
) -> i64 {
    let cache_key = (input.clone(), fft_found, dac_found);

    if cache.contains_key(&cache_key) {
        return cache[&cache_key];
    }

    if input == "out" && fft_found && dac_found {
        return 1;
    }

    let outputs = inputs_to_outputs.get(input);
    if None == outputs {
        return 0;
    }

    let fft_found = if input == "fft" { true } else { fft_found };
    let dac_found = if input == "dac" { true } else { dac_found };

    let mut total = 0;
    for output in outputs.unwrap() {
        total += solve(output, inputs_to_outputs, cache, fft_found, dac_found);
    }

    let cache_key = (input.clone(), fft_found, dac_found);
    cache.insert(cache_key, total);
    return total;
}
