use std::io;
use std::cmp::min;
use std::cmp::max;
use std::collections::VecDeque;

fn spiral_to_cart(n: u32) -> (i32, i32) {
    if n == 1 {
        return (0, 0);
    }

    // level of the spiral on which n can be found
    // 0 = 1
    // 1 = [2 .. 9]
    // 2 = [10 .. 25]
    // simple formula, much math behind it
    let lvl: i32 = (((n as f32).sqrt() - 3.0) / 2.0).ceil() as i32 + 1;

    let width = 2 * lvl + 1;

    // first element on the level lvl of the spiral
    let base = 2 + 4 * lvl * (lvl - 1);

    // q = which edge is n on
    //     0: right
    //     1: top
    //     2: left
    //     3: bottom
    //
    // Example:
    // 1 1 1 1 0
    // 2       0
    // 2       0
    // 2       0
    // 2 3 3 3 3
    let q = (n as i32 - base) / (width - 1);

    // r = indexes n on the edge r
    //
    // Example:
    // 3 2 1 0 3
    // 0       2
    // 1       1
    // 2       0
    // 3 0 1 2 3
    let r = (n as i32 - base) % (width - 1);

    let a = lvl;
    let b = 1 - lvl + r;

    // compute (x, y) from (q, r)
    match q {
        0 => (a, b),
        1 => (-b, a),
        2 => (-a, -b),
        3 => (b, -a),
        _ => panic!("Unexpected quotient {}", q),
    }
}

fn spiral_greater_than(n: u32) -> u32 {
    // Idea: walk around spiral edge by edge, summing up the elements in the
    // edge neighbouring each element of the walk in an accumulator. It needs
    // only the latest level of the spiral at each iteration so it can be done
    // using a queue.
    //
    // It requires some fix-ups to add elements around corners.

    let mut dq: VecDeque<u32> = VecDeque::new();
    let mut lvl: usize = 0;

    dq.push_back(1);

    loop {
        lvl += 1;

        let old_len = dq.len();
        let new_len = old_len + 8 * lvl;

        // reserve space for the new level of the spiral
        dq.resize(new_len, 0);

        // trick to greatly simplify the algorithm (avoids corner cases)
        // copy the last element of the spiral to the front
        {
            let tmp = dq[old_len - 1];
            dq.push_front(tmp);
        }

        // p = index of current element during the walk
        // s = accumulator
        let mut p = old_len + 1;
        let mut s = 0;

        // for each edge of the spiral
        for j in 0..4 {
            let base = j * 2 * (lvl - 1);

            for i in 0..2 * lvl {
                // range of elements of the edge that are neighbouring p
                let a = base + max(i, 1) - 1;
                let b = base + min(i + 1, 2 * (lvl - 1));

                // sum them up
                for q in a..b + 1 {
                    s += dq[q];
                }

                dq[p] = s;
                p += 1;
            }

            // add the element around the corner
            s += dq[p - 2];
        }

        // fix up bottom right corner
        dq[p - 2] += dq[old_len + 1];
        dq[p - 1] += 2 * dq[old_len + 1];

        // remove the old level of the spiral
        for _ in 0..old_len + 1 {
            dq.pop_front();
        }

        // check if an element is greater than n
        for c in &dq {
            if *c > n {
                return *c;
            }
        }
    }
}

fn mh_norm(a: (i32, i32)) -> i32 {
    a.0.abs() + a.1.abs()
}

fn main() {
    let mut input_raw = String::new();

    io::stdin().read_line(&mut input_raw).unwrap();

    let input: u32 = input_raw.trim().parse().unwrap();

    println!("Solution 1: {}", mh_norm(spiral_to_cart(input)));
    println!("Solution 2: {}", spiral_greater_than(input));
}
