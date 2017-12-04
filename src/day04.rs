use std::collections::HashSet;
use std::hash::Hash;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;

fn has_duplicates<T: Hash + Eq, I: Iterator<Item=T>>(i: I) -> bool {
    let mut set: HashSet<T> = HashSet::new();

    for x in i {
        if set.replace(x).is_some() {
            return true;
        }
    }

    false
}

fn sort_string(s: &str) -> String {
    let mut v: Vec<char> = s.chars().collect();
    v.sort_unstable();
    String::from_iter(v.iter())
}

fn main() {
    let stdin = io::stdin();

    let input: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    println!("Soltion 1: {}",
        input.iter().fold(0, |cnt, x|
            if has_duplicates(x.split_whitespace()) {
                cnt
            } else {
                cnt + 1
            }
        )
    );

    println!("Soltion 2: {}",
        input.iter().fold(0, |cnt, x|
            if has_duplicates(x.split_whitespace().map(sort_string)) {
                cnt
            } else {
                cnt + 1
            }
        )
    );



}
