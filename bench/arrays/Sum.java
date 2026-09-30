public class Sum {
    public static void main(String[] args) {
        int n = 10_000_000;
        long[] a = new long[n];
        long x = 7, total = 0;
        for (int i = 0; i < n; i++) { x = (x * 1103 + 12345) % 1_000_003; a[i] = x; }
        for (int r = 0; r < 20; r++) {
            long s = 0;
            for (long v : a) s += v;
            total += s;
        }
        System.out.println(total);
    }
}
