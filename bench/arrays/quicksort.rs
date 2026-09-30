fn quicksort(v: &mut [i64], lo: usize, hi: usize) {
    if hi - lo < 2 {
        return;
    }
    let pivot = v[hi - 1];
    let mut store = lo;
    for i in lo..hi - 1 {
        if v[i] < pivot {
            v.swap(i, store);
            store += 1;
        }
    }
    v.swap(store, hi - 1);
    quicksort(v, lo, store);
    quicksort(v, store + 1, hi);
}

fn main() {
    let n = 5_000_000;
    let mut x: i64 = 12345;
    let mut a: Vec<i64> = (0..n)
        .map(|_| {
            x = (x * 48271) % 1_000_000_007;
            x
        })
        .collect();
    quicksort(&mut a, 0, n);
    let mut check: i64 = 0;
    for v in &a {
        check = (check * 31 + (v % 1000 + 1000) % 1000) % 1_000_000_007;
    }
    println!("{}", check);
}
