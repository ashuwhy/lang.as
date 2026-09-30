function fill(n, seed) {
  const m = new Float64Array(n * n);
  let x = seed;
  for (let i = 0; i < m.length; i++) { x = (x * 37 + 11) % 1000; m[i] = x; }
  return m;
}

function matmul(a, b, n) {
  const c = new Float64Array(n * n);
  for (let i = 0; i < n; i++)
    for (let j = 0; j < n; j++) {
      let s = 0;
      for (let k = 0; k < n; k++) s += a[i * n + k] * b[k * n + j];
      c[i * n + j] = s;
    }
  return c;
}

const n = 400;
const c = matmul(fill(n, 1), fill(n, 2), n);
let check = 0;
for (const v of c) check = (check + v) % 1000000007;
console.log(check);
