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

## Read a single value
Read a single value. For non std types use ```impl_cin_type!(type)``` before or use ```cin.read_cust_s()```.
**Note:** They also have to impl. FromStr and Debug. 
```Rust
let v: i128 = cin.read();
```

## Read a tuple
Read a tuple with variable size and custom types. (Works with all types that impl. ```FromStr``` and ```Debug```)
```Rust
let (a, b, c): (String, i128, u32) = cin.read();
let (d, e): (i32, i32) = cin.read();
```

## Read a vector of tuples
Read a vector of tuples. 3 is the number of tuples in the vector to read. (Works with all types that impl. ```FromStr``` and ```Debug```)
```Rust
let vec: Vec<(u32, String)> = cin.read_vec(3);
```

## Read a vector of single values
Read a vector of single values. 3 is the number of elements to read. For non std types use ```impl_cin_type!(type)``` before or use ```cin.read_cust_v()```.
**Note:** They also have to impl. FromStr and Debug. 
```Rust
let vec: Vec<u32> = cin.read_vec(3);
```

## Iterate over input
Iterate over input. This will iterate forever. (Blocks until new input is entered) For non std types use ```impl_cin_type!(type)``` before.
```Rust
for (a, b) in cin.iter::<(String, u32)>() {
    println!("{} {}", a, b);
}
```
