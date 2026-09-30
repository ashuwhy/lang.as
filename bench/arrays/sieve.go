package main

import "fmt"

func main() {
	n := 50000000
	composite := make([]bool, n)
	count := 0
	for i := 2; i < n; i++ {
		if !composite[i] {
			count++
			for j := i * i; j < n; j += i {
				composite[j] = true
			}
		}
	}
	fmt.Println(count)
}
