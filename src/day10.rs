use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
pub fn solve(input: Vec<String>) {
    let input = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res1 = 0;

    let n = input.len();
    let m = input.len();
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    let mut scores: Vec<Vec<HashSet<u32>>> = vec![vec![HashSet::new(); m]; n];
    for i in 0..n {
        for j in 0..m {
            if input[i][j] == 9 {
                scores[i][j].insert(i as u32 * m as u32 + j as u32);
                q.push_back((i, j));
            }
        }
    }

    let are_valid_coordinates = |x: i32, y: i32| x >= 0 && x < n as i32 && y >= 0 && y < m as i32;

    let ne: Vec<i32> = vec![-1, 0, 1, 0, -1];
    while let Some(tp) = q.pop_front() {
        let (x, y) = tp;
        if input[x][y] == 0 {
            continue;
        }

        for i in 0..4 {
            let x1 = x as i32 + ne[i];
            let y1 = y as i32 + ne[i + 1];

            if are_valid_coordinates(x1, y1) && input[x1 as usize][y1 as usize] == input[x][y] - 1 {
                let new_s: HashSet<u32> = scores[x][y].iter().cloned().collect();
                scores[x1 as usize][y1 as usize].extend(new_s.iter());

                q.push_back((x1 as usize, y1 as usize));
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if input[i][j] == 0 {
                res1 += scores[i][j].len();
            }
        }
    }

    let mut res2 = 0;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut scores: Vec<Vec<u32>> = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            if input[i][j] == 9 {
                scores[i][j] = 1;
                q.push_back((i, j));
            }
        }
    }

    let are_valid_coordinates = |x: i32, y: i32| x >= 0 && x < n as i32 && y >= 0 && y < m as i32;

    let ne: Vec<i32> = vec![-1, 0, 1, 0, -1];
    while let Some(tp) = q.pop_front() {
        let (x, y) = tp;
        if input[x][y] == 0 {
            res2 += scores[x][y];
            continue;
        }

        for i in 0..4 {
            let x1 = x as i32 + ne[i];
            let y1 = y as i32 + ne[i + 1];

            if are_valid_coordinates(x1, y1) && input[x1 as usize][y1 as usize] == input[x][y] - 1 {
                if scores[x1 as usize][y1 as usize] == 0 {
                    q.push_back((x1 as usize, y1 as usize));
                }

                scores[x1 as usize][y1 as usize] += scores[x][y];
            }
        }
    }

    println!("Res1: {}, Res2: {}", res1, res2);
}
