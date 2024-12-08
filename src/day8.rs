use std::collections::HashMap;

enum Direction {
    L,
    R,
}

#[allow(dead_code)]
pub fn solve(input: Vec<String>) {
    let input = input
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let n = input.len();
    let m = input[0].len();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..n {
        for j in 0..m {
            let key = input[i][j];
            if key != '.' {
                antennas.entry(key).or_insert_with(Vec::new).push((i, j));
            }
        }
    }

    //let is_integer = |v: f64| v == v.trunc();
    let is_valid = |x: f64, y: f64| !(x < 0.0 || x >= n as f64 || y < 0.0 || y >= m as f64);

    let mut visited: Vec<bool> = vec![false; n * m];
    for c in antennas.values() {
        let sz = c.len();
        let v: Vec<_> = c.iter().collect();
        for i in 0..sz {
            for j in i + 1..sz {
                let (x1, y1) = find_new_points(
                    v[i].0 as f64,
                    v[i].1 as f64,
                    v[j].0 as f64,
                    v[j].1 as f64,
                    Direction::L,
                );

                if is_valid(x1, y1) {
                    let idx = x1 as usize * n + y1 as usize;
                    visited[idx] = true;
                }

                let (x2, y2) = find_new_points(
                    v[i].0 as f64,
                    v[i].1 as f64,
                    v[j].0 as f64,
                    v[j].1 as f64,
                    Direction::R,
                );

                if is_valid(x2, y2) {
                    let idx = x2 as usize * n + y2 as usize;
                    visited[idx] = true;
                }
            }
        }
    }

    let res1 = visited.iter().filter(|v| **v == true).count();

    let is_valid1 = |x: i32, y: i32| !(x < 0 || x >= n as i32 || y < 0 || y >= m as i32);
    let mut visited1: Vec<bool> = vec![false; n * m];
    for c in antennas.values() {
        let sz = c.len();
        let v: Vec<_> = c.iter().collect();
        for i in 0..sz {
            visited1[v[i].0 as usize * m + v[i].1 as usize] = true;
            for j in i + 1..sz {
                visited1[v[j].0 as usize * m + v[j].1 as usize] = true;
                let mut prev = (v[i].0 as i32, v[i].1 as i32);
                let dx: i32 = v[i].0 as i32 - v[j].0 as i32;
                let dy: i32 = v[i].1 as i32 - v[j].1 as i32;
                loop {
                    let (x1, y1) = (prev.0 + dx, prev.1 + dy);
                    if !is_valid1(x1, y1) {
                        break;
                    }

                    visited1[x1 as usize * m + y1 as usize] = true;
                    prev = (x1, y1);
                }

                let mut prev = (v[j].0 as i32, v[j].1 as i32);
                loop {
                    let (x1, y1) = (prev.0 - dx, prev.1 - dy);
                    if !is_valid1(x1, y1) {
                        break;
                    }

                    visited1[x1 as usize * m + y1 as usize] = true;
                    prev = (x1, y1);
                }
            }
        }
    }

    let res2 = visited1.iter().filter(|v| **v == true).count();
    //for i in 0..n {
    //    for j in 0..m {
    //        if visited1[i * m + j] {
    //            print!("#");
    //        } else {
    //            print!("{}", input[i][j]);
    //        }
    //    }
    //    println!();
    //}
    println!("Res1: {}, Res2: {}", res1, res2);
}

fn find_new_points(x1: f64, y1: f64, x2: f64, y2: f64, direction: Direction) -> (f64, f64) {
    // Step 1: Calculate the distance between the two points
    let dx = x2 - x1;
    let dy = y2 - y1;
    let distance = (dx * dx + dy * dy).sqrt();

    // Step 2: Calculate the unit vector along the line
    let ux = dx / distance;
    let uy = dy / distance;

    // Step 3: Calculate the new points
    match direction {
        Direction::L => {
            let a1_x = x1 - distance * ux;
            let a1_y = y1 - distance * uy;
            (a1_x, a1_y)
        }
        Direction::R => {
            let a2_x = x2 + distance * ux;
            let a2_y = y2 + distance * uy;
            (a2_x, a2_y)
        }
    }
}
