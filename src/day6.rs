use std::collections::{HashSet, VecDeque};

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn get(&self) -> (i32, i32) {
        match self {
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[allow(dead_code)]
pub fn solve(input: Vec<String>) {
    let mut input = input
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let n = input.len();
    let m = input[0].len();

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for i in 0..n {
        for j in 0..m {
            if input[i][j] == '^' {
                x = i as i32;
                y = j as i32;
                input[i][j] = '.';
                break;
            }
        }
    }

    let (visited_slots, _got_out) = bfs(&mut input, n, m, x, y);

    let res1 = visited_slots.len();

    let mut res2 = 0;
    for (i, j) in visited_slots {
        if !(i == x && j == y) && input[i as usize][j as usize] != '#' {
            input[i as usize][j as usize] = '#';
            if !bfs(&mut input, n, m, x, y).1 {
                res2 += 1;
            }
            input[i as usize][j as usize] = '.';
        }
    }

    println!("Res1: {}, Res2: {}", res1, res2);
}

fn bfs(
    input: &mut Vec<Vec<char>>,
    n: usize,
    m: usize,
    x: i32,
    y: i32,
) -> (HashSet<(i32, i32)>, bool) {
    let mut visited: HashSet<(i32, i32, Direction)> = HashSet::new();
    let mut visited_slots: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: VecDeque<(i32, i32, Direction)> = VecDeque::new();

    queue.push_back((x, y, Direction::Up));
    while let Some(top) = queue.pop_front() {
        visited.insert((top.0, top.1, top.2));
        visited_slots.insert((top.0, top.1));

        let mut next_x: i32 = top.0 + top.2.get().0;
        let mut next_y: i32 = top.1 + top.2.get().1;

        if next_x < 0 || next_y < 0 || next_x >= n as i32 || next_y >= m as i32 {
            return (visited_slots, true);
        }

        let mut direction = top.2;
        if input[next_x as usize][next_y as usize] == '#' {
            direction = direction.next();
            next_x = top.0;
            next_y = top.1;
        }

        if !visited.contains(&(next_x, next_y, direction)) {
            queue.push_back((next_x, next_y, direction));
        }
    }

    (visited_slots, false)
}
