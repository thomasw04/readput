# readput
[<img alt="crates.io" src="https://img.shields.io/crates/v/readput.svg">](https://crates.io/crates/readput)

Fast and easy stdin input parsing for competitive programming in rust.

# Usage
**Note:** The inputs can be on the same line, or not, it doesn't make a difference. They are separated by spaces or line breaks. Excess inputs are getting cached for the next ```read()``` call.

## Create a Scanner
Create a new ```Scanner```. (Only ASCII support for now)
```Rust
use readput::AsciiScanner;
use readput::Scanner;

let mut cin = AsciiScanner::new();
```

# Read a single value
**Note:** They have to impl. FromStr and Debug.
```Rust
let v = read!(cin, i128);
```

# Read a tuple
```Rust
let (a, b, c) = read!(cin, (String, i128, u32));
let (d, e) = read!(cin, (i32, i32));
```

# Read a vector of tuples
Read a vector of tuples. 3 is the number of tuples in the vector to read.
```Rust
let vec = read_vec!(cin, 3, (u32, String));
```

# Read a vector of single values
Read a vector of single values. 3 is the number of elements to read.
**Note:** They also have to impl. FromStr and Debug.
```Rust
let vec: Vec<u32> = read_vec!(cin, 3, u32);
```

# Iterate over input
Iterate over input. This will iterate forever. (Blocks until new input is entered).
```Rust
for (a, b) in iter!(cin, (String, u32)) {
    println!("{} {}", a, b);
}
```
