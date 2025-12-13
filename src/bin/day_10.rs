use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut expected_state_per_machine: Vec<Vec<bool>> = vec![];

    let mut buttons_per_machine: Vec<Vec<Vec<usize>>> = vec![];

    let mut expected_joltages_per_machine: Vec<Vec<usize>> = vec![];

    let file = File::open("inputs/day_10_input").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut button_indexes: Vec<Vec<usize>> = vec![];
        let mut joltages_indexes: (usize, usize) = (0, 0);

        let line = line.unwrap();
        for (i, c) in line.chars().enumerate() {
            if c == '[' {
                expected_state_per_machine.push(vec![]);
            } else if c == '.' {
                expected_state_per_machine.last_mut().unwrap().push(false);
            } else if c == '#' {
                expected_state_per_machine.last_mut().unwrap().push(true);
            } else if c == '(' {
                button_indexes.push(vec![i]);
            } else if c == ')' {
                button_indexes.last_mut().unwrap().push(i);
            } else if c == '{' {
                joltages_indexes.0 = i;
            } else if c == '}' {
                joltages_indexes.1 = i;
            }
        }

        let mut buttons: Vec<Vec<usize>> = vec![];

        for indexes in &button_indexes {
            let substring = &line[indexes[0] + 1..indexes[1]];
            let numbers = substring.split(",").map(|i| i.parse::<usize>().unwrap());
            buttons.push(numbers.collect());
        }

        buttons_per_machine.push(buttons);

        let substring = &line[joltages_indexes.0 + 1..joltages_indexes.1];
        let numbers = substring
            .split(",")
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        expected_joltages_per_machine.push(numbers);
    }

    let mut total = 0;
    for machine_index in 0..buttons_per_machine.len() {
        let mut i = 1;

        let buttons = &buttons_per_machine[machine_index];
        let expected_state = &expected_state_per_machine[machine_index];

        loop {
            let mut state: Vec<bool> = vec![false; expected_state.len()];

            let res = solve_1(buttons, &mut state, i, expected_state);
            if res {
                total += i;
                break;
            }

            i += 1;
        }
    }

    println!("{}\n\n", total);

    let mut total = 0;
    for machine_index in 0..buttons_per_machine.len() {
        let buttons = &buttons_per_machine[machine_index];
        let expected_joltages = &expected_joltages_per_machine[machine_index];
        let mut button_indexes_per_joltage: Vec<Vec<usize>> = vec![];

        let mut combinations_per_joltage: Vec<Vec<Vec<usize>>> = vec![];

        for (joltage_index, expected_joltage) in expected_joltages.iter().enumerate() {
            let button_indexes_for_joltage = buttons
                .iter()
                .enumerate()
                .filter(|(_, b)| b.contains(&joltage_index))
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();

            button_indexes_per_joltage.push(button_indexes_for_joltage.clone());

            let mut all_combinations: Vec<Vec<usize>> = vec![];
            let mut current_combination: Vec<usize> = vec![0; buttons.len()];
            find_combinations(
                *expected_joltage,
                0,
                &button_indexes_for_joltage,
                &mut current_combination,
                &mut all_combinations,
            );
            println!("{:?}", all_combinations.len());
            combinations_per_joltage.push(all_combinations);
        }

        println!("#####################");

        let mut combination_indexes_to_keep_per_joltage: Vec<HashSet<usize>> =
            vec![HashSet::new(); expected_joltages.len()];

        for (joltage_index, combinations) in combinations_per_joltage.iter().enumerate() {
            for (combination_index, combination) in combinations.iter().enumerate() {
                let mut valid = true;
                for (j_i, button_indexes) in button_indexes_per_joltage.iter().enumerate() {
                    let mut joltage_count = 0;

                    for (button_index, value) in combination.iter().enumerate() {
                        if button_indexes.contains(&button_index) {
                            joltage_count += value;
                        }
                    }

                    if joltage_count > expected_joltages[j_i] {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    combination_indexes_to_keep_per_joltage[joltage_index]
                        .insert(combination_index);
                }
            }

            println!(
                "{:?}",
                combination_indexes_to_keep_per_joltage[joltage_index].len()
            );
        }

        for joltage_index in 0..combinations_per_joltage.len() {
            let indexes_to_keep = &combination_indexes_to_keep_per_joltage[joltage_index];
            let mut idx = 0;
            combinations_per_joltage[joltage_index].retain(|_| {
                let keep = indexes_to_keep.contains(&idx);
                idx += 1;
                keep
            });

            // combinations_per_joltage[joltage_index] = combinations_per_joltage[joltage_index]
            //     .iter()
            //     .enumerate()
            //     .filter(|(i, v)| indexes_to_keep.contains(i))
            //     .map(|(i, v)| v)
            //     .collect::<Vec<Vec<usize>>>();
        }

        return;
        let mut tot = 0;
        let mut n_valid = 0;
        for joltage_i in 0..expected_joltages.len() {
            for joltage_j in (joltage_i + 1)..expected_joltages.len() {
                // rust shanenings
                let (left, right) = combinations_per_joltage.split_at_mut(joltage_j);
                let combinations_i = &mut left[joltage_i];
                let combinations_j = &mut right[0];

                let mut valid_combinations_i: HashSet<Vec<usize>> = HashSet::new();
                let mut valid_combinations_j: HashSet<Vec<usize>> = HashSet::new();

                for combination_i in combinations_i.iter_mut() {
                    for combination_j in combinations_j.iter_mut() {
                        let mut valid = true;
                        // two combinations are invalid if any button in one combination that affects the other joltage is > the value of the same button in the other combination
                        for (button_index, value_i) in combination_i.iter().enumerate() {
                            if button_indexes_per_joltage[joltage_j].contains(&button_index) {
                                if *value_i > combination_j[button_index] {
                                    valid = false;
                                    tot += 1;
                                    break;
                                }
                            }
                        }

                        if valid {
                            // valid_combinations_i.insert(combination_i.clone());
                            // valid_combinations_j.insert(combination_j.clone());
                            n_valid += 1;
                        }
                    }
                }
                println!(
                    "invalid: {}, valid: {}, valid_i: {}, valid_j: {}",
                    tot,
                    n_valid,
                    valid_combinations_i.len(),
                    valid_combinations_j.len(),
                );

                // return;
            }
        }

        // loop {
        //     println!("{}", i);

        //     let mut joltages: Vec<usize> = vec![0; expected_joltages.len()];

        //     let res = solve_2(buttons, &mut joltages, i, expected_joltages);
        //     if res {
        //         total += i;
        //         break;
        //     }

        //     i += 1;
        // }
    }

    // println!("{}", total);
}

fn solve_1(
    buttons: &Vec<Vec<usize>>,
    state: &mut Vec<bool>,
    moves_left: usize,
    expected_state: &Vec<bool>,
) -> bool {
    if moves_left == 0 {
        return state == expected_state;
    }

    for button in buttons {
        for index in button {
            let index = *index as usize;
            state[index] = !state[index];
        }

        let res = solve_1(buttons, state, moves_left - 1, expected_state);

        if res {
            return true;
        }

        for index in button {
            let index = *index as usize;
            state[index] = !state[index];
        }
    }

    false
}

fn find_combinations(
    left: usize,
    current_index_index: usize, // xD
    indexes: &Vec<usize>,
    current_combination: &mut Vec<usize>,
    all_combinations: &mut Vec<Vec<usize>>,
) {
    if current_index_index >= indexes.len() {
        all_combinations.push(current_combination.clone());
        return;
    }

    let current_index = indexes[current_index_index];

    for i in 0..=left {
        current_combination[current_index] = i;
        find_combinations(
            left - i,
            current_index_index + 1,
            indexes,
            current_combination,
            all_combinations,
        );
    }
}

fn solve_2(
    buttons: &Vec<Vec<usize>>,
    joltages: &mut Vec<usize>,
    moves_left: usize,
    expected_joltages: &Vec<usize>,
) -> bool {
    if moves_left == 0 {
        return joltages == expected_joltages;
    }

    for button in buttons {
        for index in button {
            let index = *index as usize;
            joltages[index] += 1;
        }

        for index in 0..joltages.len() {
            if joltages[index] > expected_joltages[index] {
                //revert
                for index in button {
                    let index = *index as usize;
                    joltages[index] -= 1;
                }

                return false;
            }
        }

        let res = solve_2(buttons, joltages, moves_left - 1, expected_joltages);

        if res {
            return true;
        }

        for index in button {
            let index = *index as usize;
            joltages[index] -= 1;
        }
    }

    false
}
