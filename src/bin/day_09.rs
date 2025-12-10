use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day_09_input").unwrap();
    let reader = BufReader::new(file);

    let numbers: Vec<(i64, i64)> = reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let it = line.split(',').collect::<Vec<&str>>();
            (it[1].parse::<i64>().unwrap(), it[0].parse::<i64>().unwrap())
        })
        .collect();

    // map numbers to a much small cordinate system
    // we get each unique_i and unique_j sort each of them; then each number becomes the indexes of i and j
    // bad naming and im sure it could be nicer but eh its aoc
    let (numbers, unique_i, unique_j) = {
        let mut unique_i: Vec<i64> = numbers.iter().map(|n| n.0).collect();
        let mut unique_j: Vec<i64> = numbers.iter().map(|n| n.1).collect();

        unique_i.sort_unstable();
        unique_i.dedup();
        unique_j.sort_unstable();
        unique_j.dedup();

        let map_i: HashMap<_, _> = unique_i
            .iter()
            .copied()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();

        let map_j: HashMap<_, _> = unique_j
            .iter()
            .copied()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();

        let numbers = numbers
            .iter()
            .map(|&(i, j)| (map_i[&i] as i64, map_j[&j] as i64))
            .collect::<Vec<_>>();

        (numbers, unique_i, unique_j)
    };

    // then get the borders of the polygon and check for every possible point is it inside => it is inside if we raycast in every (4) direction and hit a border
    // I am sure this can be smarter but its instant once the coordinate system is smaller
    let borders = get_polygon_borders(&numbers);

    let max_i = numbers.iter().map(|n| n.0).max().unwrap();
    let max_j = numbers.iter().map(|n| n.1).max().unwrap();

    let mut inside: HashSet<(i64, i64)> = HashSet::new();

    for i in 0..max_i {
        for j in 0..max_j {
            let p = (i, j);

            if borders.contains(&p) {
                continue;
            }

            let ok = hits_border(p, (1, 0), max_i, max_j, &borders)
                && hits_border(p, (-1, 0), max_i, max_j, &borders)
                && hits_border(p, (0, 1), max_i, max_j, &borders)
                && hits_border(p, (0, -1), max_i, max_j, &borders);

            if ok {
                inside.insert(p);
            }
        }
    }

    let mut max_area: i64 = 0;

    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            // get the all points across the rectangle borders; if borders are fully contained in the polygon then the inside must be also, also this im sure there is a smarter way
            let points = get_rectangle_borders(numbers[i], numbers[j]);

            if !points
                .iter()
                .all(|p| inside.contains(p) || borders.contains(p))
            {
                continue;
            }

            // map back to the big coordinate system to calculate the area
            let max1 = (
                unique_i[numbers[i as usize].0 as usize],
                unique_j[numbers[i as usize].1 as usize],
            );

            let max2 = (
                unique_i[numbers[j as usize].0 as usize],
                unique_j[numbers[j as usize].1 as usize],
            );

            let area = ((max1.0 - max2.0).abs() + 1) * ((max1.1 - max2.1).abs() + 1);

            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("{}", max_area);
}

fn hits_border(
    p: (i64, i64),
    d: (i64, i64),
    max_i: i64,
    max_j: i64,
    borders: &HashSet<(i64, i64)>,
) -> bool {
    // thank you doob
    let mut p = p.clone();

    loop {
        p.0 += d.0;
        p.1 += d.1;

        if p.0 < 0 || p.0 > max_i || p.1 < 0 || p.1 > max_j {
            return false;
        }

        if borders.contains(&p) {
            return true;
        }
    }
}

fn get_rectangle_borders(n1: (i64, i64), n2: (i64, i64)) -> Vec<(i64, i64)> {
    let (i1, j1) = n1;
    let (i2, j2) = n2;

    let min_i = i1.min(i2);
    let max_i = i1.max(i2);
    let min_j = j1.min(j2);
    let max_j = j1.max(j2);

    let mut points = Vec::new();

    for j in min_j..=max_j {
        points.push((min_i, j));
        points.push((max_i, j));
    }

    for i in (min_i + 1)..(max_i) {
        points.push((i, min_j));
        points.push((i, max_j));
    }

    points
}

fn get_polygon_borders(numbers: &Vec<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut borders: HashSet<(i64, i64)> = HashSet::new();

    for i in 0..numbers.len() {
        let n1 = numbers[i];
        let n2 = if i + 1 == numbers.len() {
            numbers[0]
        } else {
            numbers[i + 1]
        };

        if n1.0 == n2.0 {
            let (min, max) = (n1.1.min(n2.1), n1.1.max(n2.1));
            borders.extend((min..=max).map(|j| (n1.0, j)));
        }

        if n1.1 == n2.1 {
            let (min, max) = (n1.0.min(n2.0), n1.0.max(n2.0));
            borders.extend((min..=max).map(|i| (i, n1.1)));
        }
    }

    borders
}
