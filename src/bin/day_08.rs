use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn distance(&self, other: Pos) -> f64 {
        let v = (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2);
        (v as f64).sqrt()
    }
}

fn main() {
    let file = File::open("inputs/day_08_input").unwrap();
    let reader = BufReader::new(file);

    let positions = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let axes = l
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            Pos {
                x: axes[0],
                y: axes[1],
                z: axes[2],
            }
        })
        .collect::<Vec<Pos>>();

    let mut distances: HashMap<(&Pos, &Pos), f64> = HashMap::new();
    for (i, pos1) in positions.iter().enumerate() {
        for (j, pos2) in positions.iter().enumerate() {
            if i == j {
                continue;
            }

            if distances.contains_key(&(pos1, pos2)) || distances.contains_key(&(pos2, pos1)) {
                continue;
            }

            distances.insert((pos1, pos2), pos1.distance(pos2.clone()));
        }
    }

    let mut pairs_by_distance: Vec<(&Pos, &Pos)> = distances.keys().cloned().collect();
    pairs_by_distance.sort_by(|a, b| distances[b].partial_cmp(&distances[a]).unwrap()); // reverse since its cheaper to pop from the end then the start

    let mut pos_to_circuit: HashMap<Pos, i32> = positions
        .iter()
        .enumerate()
        .map(|(i, pos)| (pos.clone(), i as i32))
        .collect();

    loop {
        let next = pairs_by_distance.pop().unwrap();

        let (circuit_1, circuit_2) = (
            pos_to_circuit.get(&next.0).copied(),
            pos_to_circuit.get(&next.1).copied(),
        );

        let pos_to_circuit = &mut pos_to_circuit;

        //combine the circuits
        if let (Some(c1), Some(c2)) = (circuit_1, circuit_2) {
            for v in pos_to_circuit.values_mut() {
                if *v == c2 {
                    *v = c1;
                }
            }
        } else if let (Some(c1), None) = (circuit_1, circuit_2) {
            pos_to_circuit.insert(next.1.clone(), c1);
        } else if let (None, Some(c2)) = (circuit_1, circuit_2) {
            pos_to_circuit.insert(next.0.clone(), c2);
        } else {
        }

        let all_same = {
            let mut iter = pos_to_circuit.values();
            if let Some(first) = iter.next() {
                iter.all(|v| v == first)
            } else {
                true
            }
        };

        if all_same {
            println!("{}", next.0.x * next.1.x);
            break;
        }
    }

    // P1 (change the loop to 10 iterations)
    // let mut value_counts: HashMap<i32, usize> = HashMap::new();
    // for (_k, v) in &pos_to_circuit {
    //     *value_counts.entry(*v).or_insert(0) += 1;
    // }

    // let mut values = value_counts.values().cloned().collect::<Vec<usize>>();

    // values.sort_by(|a, &b| b.cmp(a));

    // let res = values.iter().take(3).fold(1, |a, &b| a * b);

    // println!("{:?}", res);
}
