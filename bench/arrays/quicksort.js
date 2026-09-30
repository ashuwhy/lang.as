function quicksort(v, lo, hi) {
  if (hi - lo < 2) return;
  const pivot = v[hi - 1];
  let store = lo;
  for (let i = lo; i < hi - 1; i++)
    if (v[i] < pivot) { const t = v[i]; v[i] = v[store]; v[store] = t; store++; }
  const t = v[store]; v[store] = v[hi - 1]; v[hi - 1] = t;
  quicksort(v, lo, store);
  quicksort(v, store + 1, hi);
}

const n = 5_000_000;
const a = new Float64Array(n);
let x = 12345;
for (let i = 0; i < n; i++) { x = (x * 48271) % 1000000007; a[i] = x; }
quicksort(a, 0, n);
let check = 0;
for (const v of a) check = (check * 31 + ((v % 1000) + 1000) % 1000) % 1000000007;
console.log(check);
