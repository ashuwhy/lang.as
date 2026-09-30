fn main() {
    let n = 10_000_000;
    let mut x: i64 = 7;
    let a: Vec<i64> = (0..n)
        .map(|_| {
            x = (x * 1103 + 12345) % 1_000_003;
            x
        })
        .collect();
    let mut total: i64 = 0;
    for _ in 0..20 {
        let s: i64 = a.iter().sum();
        total += s;
    }
    println!("{}", total);
}
