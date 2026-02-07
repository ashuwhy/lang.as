package main

import "C"
import (
    "sync"
)

//export ConcurrentMap
func ConcurrentMap(data []float64, fn func(float64) float64) []float64 {
    result := make([]float64, len(data))
    var wg sync.WaitGroup
    
    chunks := len(data) / 4
    for i := 0; i < 4; i++ {
        wg.Add(1)
        start := i * chunks
        end := start + chunks
        if i == 3 {
            end = len(data)
        }
        
        go func(start, end int) {
            defer wg.Done()
            for j := start; j < end; j++ {
                result[j] = fn(data[j])
            }
        }(start, end)
    }
    
    wg.Wait()
    return result
}

func main() {} 