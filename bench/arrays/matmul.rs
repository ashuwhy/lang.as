fn fill(n: usize, seed: i64) -> Vec<i64> {
    let mut x = seed;
    (0..n * n)
        .map(|_| {
            x = (x * 37 + 11) % 1000;
            x
        })
        .collect()
}

fn matmul(a: &[i64], b: &[i64], n: usize) -> Vec<i64> {
    let mut c = vec![0i64; n * n];
    for i in 0..n {
        for j in 0..n {
            let mut s = 0;
            for k in 0..n {
                s += a[i * n + k] * b[k * n + j];
            }
            c[i * n + j] = s;
        }
    }
    c
}

fn main() {
    let n = 400;
    let c = matmul(&fill(n, 1), &fill(n, 2), n);
    let mut check = 0i64;
    for v in &c {
        check = (check + v) % 1_000_000_007;
    }
    println!("{}", check);
}
