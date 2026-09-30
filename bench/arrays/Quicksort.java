public class Quicksort {
    static void quicksort(long[] v, int lo, int hi) {
        if (hi - lo < 2) return;
        long pivot = v[hi - 1];
        int store = lo;
        for (int i = lo; i < hi - 1; i++)
            if (v[i] < pivot) { long t = v[i]; v[i] = v[store]; v[store] = t; store++; }
        long t = v[store]; v[store] = v[hi - 1]; v[hi - 1] = t;
        quicksort(v, lo, store);
        quicksort(v, store + 1, hi);
    }

    public static void main(String[] args) {
        int n = 5_000_000;
        long[] a = new long[n];
        long x = 12345, check = 0;
        for (int i = 0; i < n; i++) { x = (x * 48271) % 1_000_000_007L; a[i] = x; }
        quicksort(a, 0, n);
        for (long v : a) check = (check * 31 + (v % 1000 + 1000) % 1000) % 1_000_000_007L;
        System.out.println(check);
    }
}
