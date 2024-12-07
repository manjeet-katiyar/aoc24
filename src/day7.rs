use std::usize;

enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn get(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Add => left + right,
            Self::Mul => left * right,
            Self::Concat => format!("{}{}", left, right).parse().unwrap(),
        }
    }
}

#[allow(dead_code)]
pub fn solve(input: Vec<String>) {
    let res1: i64 = input
        .iter()
        .map(|line| {
            let temp: Vec<i64> = line
                .split(&[':', ' '][..])
                .filter(|val| !val.is_empty())
                .map(|val| match val.parse() {
                    Ok(v) => v,
                    Err(error) => panic!("V: {}, error: {:?}", val, error),
                })
                .collect();
            if is_valid(
                &[Operator::Add, Operator::Mul],
                &temp[1..].into(),
                temp[0],
                temp[1],
                1,
            ) {
                temp[0]
            } else {
                0
            }
        })
        .sum();

    let res2: i64 = input
        .iter()
        .map(|line| {
            let temp: Vec<i64> = line
                .split(&[':', ' '][..])
                .filter(|val| !val.is_empty())
                .map(|val| match val.parse() {
                    Ok(v) => v,
                    Err(error) => panic!("V: {}, error: {:?}", val, error),
                })
                .collect();
            if is_valid(
                &[Operator::Add, Operator::Mul, Operator::Concat],
                &temp[1..].into(),
                temp[0],
                temp[1],
                1,
            ) {
                temp[0]
            } else {
                0
            }
        })
        .sum();

    println!("Res1: {}, Res2: {}", res1, res2);
}

fn is_valid(ops: &[Operator], nums: &Vec<i64>, target: i64, current: i64, index: usize) -> bool {
    if index >= nums.len() {
        return target == current;
    }

    if current > target {
        return false;
    }

    for op in ops {
        if is_valid(&ops, &nums, target, op.get(current, nums[index]), index + 1) {
            return true;
        }
    }

    return false;
}
