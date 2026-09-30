const n = 10_000_000;
const a = new Float64Array(n);
let x = 7;
for (let i = 0; i < n; i++) { x = (x * 1103 + 12345) % 1000003; a[i] = x; }
let total = 0;
for (let r = 0; r < 20; r++) {
  let s = 0;
  for (let i = 0; i < n; i++) s += a[i];
  total += s;
}
console.log(total);
