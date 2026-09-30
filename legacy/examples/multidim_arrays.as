// 3D Array example
cube = array[2][2][2]
cube[0][0][0] = 1
cube[0][0][1] = 2
cube[0][1][0] = 3
cube[0][1][1] = 4
cube[1][0][0] = 5
cube[1][0][1] = 6
cube[1][1][0] = 7
cube[1][1][1] = 8

// Print 3D array elements
i = 0
while i < 2
do {
    j = 0
    while j < 2
    do {
        k = 0
        while k < 2
        do {
            output cube[i][j][k]
            inc k
        }
        inc j
    }
    inc i
}

// 4D Array example
hypercube = array[2][2][2][2]
hypercube[0][0][0][0] = 1
hypercube[1][1][1][1] = 16

// Access specific 4D array element
output "Value at [1][1][1][1]: "
output hypercube[1][1][1][1] 