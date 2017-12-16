use std::io;

fn solve_captcha1(s: &str) -> u32 {
    let mut sum = 0;
    let chars = s.chars().chain(s.chars().take(1));
    let mut prev_char = '\0';

    for c in chars {
        if c == prev_char {
            sum += c.to_digit(10).expect("Captcha is invalid");
        }

        prev_char = c;
    }

    sum
}

fn solve_captcha2(s: &str) -> u32 {
    let mut sum = 0;
    let len = s.chars().count();
    let chars2 = s.chars().skip(len / 2).chain(s.chars().take(len / 2));

    for (c1, c2) in s.chars().zip(chars2) {
        if c1 == c2 {
            sum += c1.to_digit(10).expect("Captcha is invalid");
        }
    }

    sum
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    println!("Solution 1: {}", solve_captcha1(input));
    println!("Solution 2: {}", solve_captcha2(input));
}
