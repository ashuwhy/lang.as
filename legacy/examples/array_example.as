// 1D Array example
numbers = array[5]
numbers[0] = 10
numbers[1] = 20
numbers[2] = 30
numbers[3] = 40
numbers[4] = 50

// Print array elements
i = 0
while i < 5
do {
    output numbers[i]
    inc i
}

// 2D Array example
matrix = array[3][3]
matrix[0][0] = 1
matrix[0][1] = 2
matrix[0][2] = 3
matrix[1][0] = 4
matrix[1][1] = 5
matrix[1][2] = 6
matrix[2][0] = 7
matrix[2][1] = 8
matrix[2][2] = 9

// Print matrix elements
row = 0
while row < 3
do {
    col = 0
    while col < 3
    do {
        output matrix[row][col]
        inc col
    }
    inc row
} 