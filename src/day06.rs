use std::io;

fn step(v: &mut Vec<i32>) {
    let mut longest_i = 0;
    let mut longest = v[0];

    for i in 1..v.len() {
        if v[i] > longest {
            longest_i = i;
            longest = v[i];
        }
    }

    v[longest_i] = 0;

    while longest > 0 {
        longest_i = (longest_i + 1) % v.len();
        v[longest_i] += 1;
        longest -= 1;
    }
}

fn find_cycle(init: &Vec<i32>) -> (u32, u32) {
    let mut iters: Vec<Vec<i32>> = Vec::new();

    iters.push(init.to_vec());

    loop {
        let mut last = iters.last().unwrap().to_vec();

        step(&mut last);

        for i in 0..iters.len() {
            if iters[i] == last {
                return (iters.len() as u32, (iters.len() - i) as u32);
            }
        }

        iters.push(last);
    }
}

fn main() {
    let mut input_raw: String = String::new();

    io::stdin().read_line(&mut input_raw).unwrap();

    let input: Vec<i32> = input_raw
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();

    let (s1, s2) = find_cycle(&input);

    println!("Solution 1: {}", s1);
    println!("Solution 2: {}", s2);
}
