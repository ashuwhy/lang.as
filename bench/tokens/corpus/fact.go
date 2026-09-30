func Factorial(n uint64) uint64 {
	if n == 0 {
		return 1
	}
	return n * Factorial(n-1)
}
