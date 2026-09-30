fn main() {
    let n: usize = 50_000_000;
    let mut composite = vec![false; n];
    let mut count: i64 = 0;
    for i in 2..n {
        if !composite[i] {
            count += 1;
            let mut j = i * i;
            while j < n {
                composite[j] = true;
                j += i;
            }
        }
    }
    println!("{}", count);
}
