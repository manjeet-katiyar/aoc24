use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

#[allow(dead_code)]
pub fn solve(input: Vec<String>) {
    let input = input
        .split(|line| line.trim().is_empty())
        .collect::<Vec<_>>();

    let rules = input[0].iter().fold(
        HashMap::<String, HashSet<String>>::new(),
        |mut acc, rule| {
            let rule = rule.split('|').collect::<Vec<_>>();
            acc.entry(rule[0].to_string())
                .or_insert_with(HashSet::new)
                .insert(rule[1].to_string());

            acc
        },
    );

    let res1: i32 = input[1]
        .iter()
        .map(|line| {
            let vals = line
                .split(',')
                .map(|item| item.to_string())
                .collect::<Vec<_>>();
            //dbg!(&vals, is_valid(&rules, &vals));
            if is_valid(&rules, &vals) {
                vals[vals.len() / 2 as usize].parse::<i32>().unwrap()
            } else {
                0
            }
        })
        .sum();

    //dbg!(&topo);

    let res2: i32 = input[1]
        .iter()
        .map(|line| {
            let vals = line
                .split(',')
                .map(|item| item.to_string())
                .collect::<Vec<_>>();

            let new_rules: HashMap<_, HashSet<_>> = rules
                .iter()
                .filter(|(key, _)| vals.contains(&key))
                .map(|(key, value)| {
                    (
                        key.clone(),
                        value
                            .iter()
                            .filter(|&x| vals.contains(&x))
                            .cloned()
                            .collect(),
                    )
                })
                .collect();

            let mut in_degree =
                new_rules
                    .iter()
                    .fold(HashMap::<String, i32>::new(), |mut acc, (bef, afters)| {
                        for after in afters {
                            let counter = acc.entry(after.to_string()).or_insert(0);
                            *counter += 1;
                        }

                        acc.entry(bef.to_string()).or_insert(0);

                        acc
                    });

            let mut queue = VecDeque::new();

            for k in in_degree.keys().cloned() {
                if let Some(0) = in_degree.get(&k) {
                    queue.push_back(k);
                }
            }

            let mut topo: HashMap<String, i32> = HashMap::new();
            let mut ind: i32 = 0;

            while let Some(top) = queue.pop_front() {
                topo.insert(top.clone(), ind);
                ind += 1;
                for rule in rules.get(&top).unwrap_or(&HashSet::new()) {
                    if in_degree.get(rule).is_none() {
                        continue;
                    }

                    if let Some(0) = in_degree.get(rule) {
                        continue;
                    }

                    in_degree.entry(rule.to_string()).and_modify(|v| *v -= 1);
                    if let Some(0) = in_degree.get(rule) {
                        queue.push_back(rule.to_string());
                    }
                }
            }

            if is_valid(&rules, &vals) {
                0
            } else {
                let mut temp: Vec<(i32, String)> = vec![];
                for val in vals {
                    if let Some(ind) = topo.get(&val) {
                        temp.push((*ind, val));
                    }
                }

                temp.sort();

                if temp.len() > 0 {
                    temp[temp.len() / 2 as usize].1.parse::<i32>().unwrap()
                } else {
                    0
                }
            }
        })
        .sum();
    println!("Res1: {}, Res2: {}", res1, res2);
}

fn is_valid(rules: &HashMap<String, HashSet<String>>, update: &Vec<String>) -> bool {
    let mut visited: HashSet<String> = HashSet::new();
    for page in update {
        let has_intersection = rules
            .get(page)
            .map(|rules| visited.intersection(rules).count() > 0)
            .unwrap_or(false);

        if has_intersection {
            return false;
        }

        visited.insert(page.to_string());
    }

    return true;
}
