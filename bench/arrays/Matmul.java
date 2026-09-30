public class Matmul {
    static long[] fill(int n, long seed) {
        long[] m = new long[n * n];
        long x = seed;
        for (int i = 0; i < m.length; i++) { x = (x * 37 + 11) % 1000; m[i] = x; }
        return m;
    }

    static long[] matmul(long[] a, long[] b, int n) {
        long[] c = new long[n * n];
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++) {
                long s = 0;
                for (int k = 0; k < n; k++) s += a[i * n + k] * b[k * n + j];
                c[i * n + j] = s;
            }
        return c;
    }

    public static void main(String[] args) {
        int n = 400;
        long[] c = matmul(fill(n, 1), fill(n, 2), n);
        long check = 0;
        for (long v : c) check = (check + v) % 1_000_000_007L;
        System.out.println(check);
    }
}
