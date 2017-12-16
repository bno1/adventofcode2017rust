use std::io;
use std::io::BufRead;
use std::cmp;

fn parse_line(l: &str) -> Vec<i32> {
    l.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn min_max(l: &Vec<i32>) -> i32 {
    l.iter()
        .fold(None, |mm, x| {
            mm.map(|(mi, ma)| (cmp::min(mi, x), cmp::max(ma, x)))
                .or_else(|| Some((x, x)))
        })
        .map_or(0, |(mi, ma)| ma - mi)
}

fn quotient(l: &Vec<i32>) -> i32 {
    for i in 0..l.len() {
        for j in (i + 1)..l.len() {
            let a = cmp::max(l[i], l[j]);
            let b = cmp::min(l[i], l[j]);

            if a % b == 0 {
                return a / b;
            }
        }
    }

    0
}

fn main() {
    let stdin = io::stdin();

    let matrix: Vec<Vec<i32>> = stdin
        .lock()
        .lines()
        .map(|l| parse_line(l.unwrap().as_str()))
        .collect();

    let minmax_sum: i32 = matrix.iter().map(min_max).sum();
    println!("Solution 1: {}", minmax_sum);

    let quot_sum: i32 = matrix.iter().map(quotient).sum();
    println!("Solution 2: {}", quot_sum);
}
