package main

import "fmt"

func main() {
	n := 10000000
	a := make([]int64, n)
	var x int64 = 7
	for i := range a {
		x = (x*1103 + 12345) % 1000003
		a[i] = x
	}
	var total int64
	for r := 0; r < 20; r++ {
		var s int64
		for _, v := range a {
			s += v
		}
		total += s
	}
	fmt.Println(total)
}
