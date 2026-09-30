package main

import "fmt"

func quicksort(v []int64, lo, hi int) {
	if hi-lo < 2 {
		return
	}
	pivot, store := v[hi-1], lo
	for i := lo; i < hi-1; i++ {
		if v[i] < pivot {
			v[i], v[store] = v[store], v[i]
			store++
		}
	}
	v[store], v[hi-1] = v[hi-1], v[store]
	quicksort(v, lo, store)
	quicksort(v, store+1, hi)
}

func main() {
	n := 5000000
	a := make([]int64, n)
	var x int64 = 12345
	for i := range a {
		x = (x * 48271) % 1000000007
		a[i] = x
	}
	quicksort(a, 0, n)
	var check int64
	for _, v := range a {
		check = (check*31 + (v%1000+1000)%1000) % 1000000007
	}
	fmt.Println(check)
}
