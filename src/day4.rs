use std::usize;

#[allow(dead_code)]
pub fn solve(input: Vec<String>) {
    let input = input
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res1 = 0;

    let n = input.len() as i32;
    let m = input[0].len() as i32;

    for i in 0..n {
        for j in 0..m {
            res1 += exists(&input, n, m, i, j, -1, 0);
            res1 += exists(&input, n, m, i, j, -1, 1);
            res1 += exists(&input, n, m, i, j, 0, 1);
            res1 += exists(&input, n, m, i, j, 1, 1);
            res1 += exists(&input, n, m, i, j, 1, 0);
            res1 += exists(&input, n, m, i, j, 1, -1);
            res1 += exists(&input, n, m, i, j, 0, -1);
            res1 += exists(&input, n, m, i, j, -1, -1);
        }
    }

    let mut res2 = 0;
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            let x = i as usize;
            let y = j as usize;
            if input[x][y] == 'A' {
                let a = (input[x - 1][y - 1] == 'M' && input[x + 1][y + 1] == 'S')
                    || (input[x - 1][y - 1] == 'S' && input[x + 1][y + 1] == 'M');

                let b = (input[x - 1][y + 1] == 'M' && input[x + 1][y - 1] == 'S')
                    || (input[x - 1][y + 1] == 'S' && input[x + 1][y - 1] == 'M');

                if a && b {
                    res2 += 1;
                }
            }
        }
    }
    println!("Res1: {}, Res2: {}", res1, res2);
}

fn exists(arr: &Vec<Vec<char>>, n: i32, m: i32, mut x: i32, mut y: i32, dx: i32, dy: i32) -> i32 {
    let expected = vec!['X', 'M', 'A', 'S'];
    let mut ind = 0;
    loop {
        if ind >= expected.len() {
            break;
        }

        if x < 0 || y < 0 || x >= n || y >= m {
            return 0;
        }

        if arr[x as usize][y as usize] != expected[ind] {
            return 0;
        }

        ind += 1;

        x += dx;
        y += dy;
    }

    return 1;
}
