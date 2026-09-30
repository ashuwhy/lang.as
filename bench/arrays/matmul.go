package main

import "fmt"

func fill(n int, seed int64) []int64 {
	m := make([]int64, n*n)
	x := seed
	for i := range m {
		x = (x*37 + 11) % 1000
		m[i] = x
	}
	return m
}

func matmul(a, b []int64, n int) []int64 {
	c := make([]int64, n*n)
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			var s int64
			for k := 0; k < n; k++ {
				s += a[i*n+k] * b[k*n+j]
			}
			c[i*n+j] = s
		}
	}
	return c
}

func main() {
	n := 400
	c := matmul(fill(n, 1), fill(n, 2), n)
	var check int64
	for _, v := range c {
		check = (check + v) % 1000000007
	}
	fmt.Println(check)
}
