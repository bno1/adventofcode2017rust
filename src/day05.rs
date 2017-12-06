use std::io;
use std::io::BufRead;


fn find_exit<'a, T: Iterator<Item=&'a i32>>(t: T, f: fn(i32) -> i32) -> u32 {
    let mut v: Vec<i32> = t.cloned().collect();

    let mut i: i32 = 0;
    let mut cnt: u32 = 0;

    while i >= 0 && i < v.len() as i32 {
        let x = v[i as usize];
        v[i as usize] = f(x);
        i += x;
        cnt += 1;
    }

    cnt
}

fn main() {
    let stdin = io::stdin();

    let input: Vec<i32> = stdin.lock()
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    println!("Solution 1: {}", find_exit(input.iter(), |x| x + 1));
    println!("Solution 2: {}", find_exit(input.iter(), |x|
        if x == 3 {
            x - 1
        } else {
            x + 1
        })
    );
}
