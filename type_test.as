// Type annotation test
let x: Number = 10;
let y: Number = 20;
let name: String = "AS Lang";

output x + y;
output name;

// Type inference (no annotation)
let z = x * 2;
output z;

// Boolean type
let flag: Boolean = true;
if flag {
    output "Flag is true";
}

output "Type checking complete!";
