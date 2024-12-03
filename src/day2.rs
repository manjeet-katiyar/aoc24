#[allow(dead_code)]
pub fn solve(input: Vec<String>) {
    let res1 = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|word| i32::from_str_radix(word, 10).unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|level| is_valid_level(level))
        .count();

    let res2 = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|word| i32::from_str_radix(word, 10).unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|level| {
            let mut critical_points = vec![0, level.len() - 1];
            for i in 1..level.len() - 1 {
                let left = level[i] - level[i - 1];
                let right = level[i + 1] - level[i];

                let valid = left.abs() >= 1
                    && left.abs() < 4
                    && right.abs() >= 1
                    && right.abs() < 4
                    && ((left > 0 && right > 0) || (left < 0 && right < 0));

                if !valid {
                    critical_points.push(i);
                }
            }

            if critical_points.len() == 0 {
                return true;
            }

            for ind in critical_points {
                if is_valid_level(&[&level[..ind], &level[ind + 1..]].concat()) {
                    return true;
                }
            }

            false
        })
        .count();

    println!("Res1: {}, Res2: {}", res1, res2);
}

fn is_valid_level(level: &Vec<i32>) -> bool {
    let mut inc: Option<bool> = None;
    for i in 1..level.len() {
        inc = match inc {
            None => Some(level[i] - level[i - 1] > 0),
            inc => inc,
        };

        let curr = level[i] - level[i - 1];
        let is_inc = curr > 0;
        if is_inc != inc.unwrap() || curr.abs() < 1 || curr.abs() > 3 {
            return false;
        }
    }

    true
}
