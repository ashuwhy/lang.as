// Longest Collatz chain for a start below 3,000,000.
fn chain(start: i64) -> i64 {
    let mut x = start;
    let mut steps = 1;
    while x != 1 {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
        steps += 1;
    }
    steps
}

fn main() {
    let mut best = 0;
    let mut best_start = 0;
    let mut n = 1;
    while n < 3_000_000 {
        let c = chain(n);
        if c > best {
            best = c;
            best_start = n;
        }
        n += 1;
    }
    println!("{} {}", best_start, best);
}
