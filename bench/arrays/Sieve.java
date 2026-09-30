public class Sieve {
    public static void main(String[] args) {
        int n = 50_000_000;
        boolean[] composite = new boolean[n];
        long count = 0;
        for (int i = 2; i < n; i++) {
            if (!composite[i]) {
                count++;
                for (long j = (long) i * i; j < n; j += i) composite[(int) j] = true;
            }
        }
        System.out.println(count);
    }
}
