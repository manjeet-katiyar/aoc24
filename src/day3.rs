use regex::Regex;

#[allow(dead_code)]
pub fn solve(input: Vec<String>) {
    let mut res1 = 0;

    let re: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    input.iter().for_each(|line| {
        let res: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
        res.iter().for_each(|mul| {
            let idx1 = mul.find(',').unwrap();
            let num1: i32 = mul[4..idx1].parse().unwrap();
            let num2: i32 = mul[idx1 + 1..mul.len() - 1].parse().unwrap();

            res1 += num1 * num2;
        });
        //println!("Vec: {:?}", res);
    });

    let mut res2 = 0;
    let mut flag = true;

    let re: Regex = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don\'t\(\))").unwrap();
    input.iter().for_each(|line| {
        let res: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();

        res.iter().for_each(|mul| {
            if *mul == "do()" {
                flag = true;
            } else if *mul == "don't()" {
                flag = false;
            } else if flag {
                let idx1 = mul.find(',').unwrap();
                let num1: i32 = mul[4..idx1].parse().unwrap();
                let num2: i32 = mul[idx1 + 1..mul.len() - 1].parse().unwrap();

                res2 += num1 * num2;
            }
        });
    });

    println!("Res1: {}, Res2: {}", res1, res2);
}
