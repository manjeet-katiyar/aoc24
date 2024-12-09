use num_bigint::BigUint;
use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    usize,
};

#[derive(Debug, Clone, Copy)]
enum Memory {
    File(u32, u8),
    Space(u8),
}

#[allow(dead_code)]
pub fn solve(input: Vec<String>) {
    let input = input[0].clone();

    let mut mem_v: Vec<Memory> = Vec::new();

    for (idx, c) in input.chars().enumerate() {
        let n: u8 = c.to_digit(10).unwrap().try_into().unwrap();
        if idx % 2 == 0 {
            let id: u32 = (idx / 2).try_into().unwrap();
            mem_v.push(Memory::File(id, n));
        } else {
            mem_v.push(Memory::Space(n));
        }
    }

    let compute = |id: u32, n: u8, a: u32| {
        let id = BigUint::from(id);
        let n = BigUint::from(n);
        let n1 = BigUint::from(n.clone());
        let a = BigUint::from(a);
        let one = BigUint::from(1 as u32);
        let two = BigUint::from(2 as u32);
        let two1 = BigUint::from(2 as u32);

        id * ((n * (two * a + n1 - one)) / two1)
    };

    let mut res1 = BigUint::ZERO;
    let mut idx = 0;
    let mut mem: VecDeque<Memory> = mem_v[..].to_vec().into();
    while let Some(tp) = mem.pop_front() {
        if let Some(Memory::File(id, blocks)) = mem.pop_back() {
            mem.push_back(Memory::File(id, blocks));
        }

        match tp {
            Memory::File(id, blocks) => {
                res1 += compute(id, blocks, idx);
                idx += blocks as u32;
            }
            Memory::Space(spaces) => {
                if let Some(Memory::File(id, blocks)) = mem.pop_back() {
                    let n = spaces.min(blocks);
                    res1 += compute(id, n, idx);
                    idx += n as u32;

                    if spaces > blocks {
                        mem.push_front(Memory::Space(spaces - n));
                    } else {
                        mem.push_back(Memory::File(id, blocks - n));
                    }
                }
            }
        }
    }

    let mut res2 = BigUint::ZERO;
    let mut idx = 0;
    let mut mem: VecDeque<Memory> = mem_v[..].to_vec().into();
    let mut file_map: Vec<BTreeSet<u32>> = vec![BTreeSet::new(); 10];
    mem_v.iter().for_each(|m| match m {
        Memory::File(_id, 0) => {}
        Memory::File(id, blocks) => {
            file_map[*blocks as usize].insert(*id);
            ()
        }
        _ => {}
    });

    let mut deleted: HashSet<u32> = HashSet::new();

    let mut tempp = String::new();
    while let Some(tp) = mem.pop_front() {
        match tp {
            Memory::File(id, blocks) if !deleted.contains(&id) => {
                tempp += vec![id.clone(); blocks as usize]
                    .iter()
                    .map(|&num| num.to_string())
                    .collect::<Vec<String>>()
                    .join("")
                    .as_str();

                res2 += compute(id, blocks, idx);
                idx += blocks as u32;
                file_map[blocks as usize].remove(&id);
            }
            Memory::File(_, spaces) => {
                tempp += vec!["."; spaces as usize].join("").as_str();
                idx += spaces as u32;
            }
            Memory::Space(spaces) => {
                //println!("{} {:?}", spaces, file_map);
                let mut mx_idx = 0;
                let mut mx = 0;
                let mut found = false;
                for i in 1..spaces + 1 {
                    if let Some(bb) = file_map[i as usize].last() {
                        if *bb > mx {
                            mx = *bb;
                            mx_idx = i;
                            found = true;
                        }
                    }
                }

                if found {
                    file_map[mx_idx as usize].remove(&mx);
                    deleted.insert(mx);

                    tempp += vec![mx.clone(); mx_idx as usize]
                        .iter()
                        .map(|&num| num.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                        .as_str();

                    res2 += compute(mx, mx_idx, idx);
                    idx += mx_idx as u32;
                    if mx_idx < spaces {
                        mem.push_front(Memory::Space(spaces - mx_idx));
                    }
                } else {
                    tempp += vec!["."; spaces as usize].join("").as_str();

                    idx += spaces as u32;
                }
            }
        }
    }

    //dbg!(&tempp);

    println!("Res1: {}, Res2: {}", res1, res2);
}
