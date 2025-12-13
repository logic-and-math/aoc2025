use std::{
    collections::{HashMap, HashSet},
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

    total = 0;
    for machine_index in 0..buttons_per_machine.len() {
        let buttons = &buttons_per_machine[machine_index];
        let expected_joltages = &expected_joltages_per_machine[machine_index];

        let mut n_presses_per_index: Vec<usize> = vec![0; buttons.len()];
        let mut indicators_to_presses: HashMap<Vec<bool>, Vec<Vec<usize>>> = HashMap::new();

        fill_indicators_to_presses(
            0,
            buttons,
            &mut n_presses_per_index,
            &mut indicators_to_presses,
            expected_joltages.len(),
        );

        let mut cache: HashMap<Vec<usize>, Option<i64>> = HashMap::new();

        let res = find_min_presses(
            expected_joltages,
            buttons,
            &indicators_to_presses,
            &mut cache,
        )
        .unwrap();

        total += res as usize;
    }

    println!("{}", total);
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

fn fill_indicators_to_presses(
    current_index: usize,
    buttons: &Vec<Vec<usize>>,
    n_presses_per_index: &mut Vec<usize>,
    indicators_to_presses: &mut HashMap<Vec<bool>, Vec<Vec<usize>>>,
    n_lights: usize,
) {
    if current_index >= buttons.len() {
        let mut indicator: Vec<bool> = vec![false; n_lights];

        for (button_index, n_presses) in n_presses_per_index.iter().enumerate() {
            let button = &buttons[button_index];
            if *n_presses == 1 {
                for light_index in button {
                    indicator[*light_index] = !indicator[*light_index]
                }
            }
        }

        if !indicators_to_presses.contains_key(&indicator) {
            indicators_to_presses.insert(indicator.clone(), vec![]);
        }

        indicators_to_presses
            .get_mut(&indicator)
            .unwrap()
            .push(n_presses_per_index.clone());

        return;
    }

    // each button can be pressed only 0 or 1 time for the indicator state
    for n_presses in 0..=1 {
        n_presses_per_index[current_index] = n_presses;
        fill_indicators_to_presses(
            current_index + 1,
            buttons,
            n_presses_per_index,
            indicators_to_presses,
            n_lights,
        );
    }
}

fn find_min_presses(
    joltages: &Vec<usize>,
    buttons: &Vec<Vec<usize>>,
    indicators_to_presses: &HashMap<Vec<bool>, Vec<Vec<usize>>>,
    cache: &mut HashMap<Vec<usize>, Option<i64>>,
) -> Option<i64> {
    if joltages.iter().all(|v| *v == 0) {
        return Some(0);
    }

    let indicator: Vec<bool> = joltages.iter().map(|j| j % 2 == 1).collect();

    let mut res: Option<i64> = None;

    if !indicators_to_presses.contains_key(&indicator) {
        return None;
    }

    for presses in &indicators_to_presses[&indicator] {
        let mut new_joltages = joltages.clone();

        let mut ok = true;

        'buttons: for (button_index, n_presses) in presses.iter().enumerate() {
            if *n_presses == 0 {
                continue;
            }

            for &joltage_index in &buttons[button_index] {
                if new_joltages[joltage_index] == 0 {
                    ok = false;
                    break 'buttons;
                }
                new_joltages[joltage_index] -= 1;
            }
        }

        if !ok {
            continue;
        }

        new_joltages = new_joltages.iter().map(|v| v / 2).collect();

        let min_presses_for_new_joltages = if cache.contains_key(&new_joltages) {
            cache[&new_joltages]
        } else {
            let v = find_min_presses(&new_joltages, buttons, indicators_to_presses, cache);
            cache.insert(new_joltages, v);
            v
        };

        if min_presses_for_new_joltages == None {
            continue;
        }

        let num_presses = (presses.iter().filter(|&&x| x > 0).count() as i64)
            + (2 * min_presses_for_new_joltages.unwrap());

        res = match res {
            None => Some(num_presses),
            Some(r) => Some(num_presses.min(r)),
        };
    }

    return res;
}
