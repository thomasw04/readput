use readput::AsciiScanner;
use readput::Scanner;

fn main() {
    let mut cin = AsciiScanner::new();

    //Read a single value. For non std types use impl_cin_type(type)! before.
    let a: i128 = cin.read();

    //Read a tuple with variable size and custom types (need to impl FromStr and Debug). Works with all types.
    let (b, c, d): (String, i128, u32) = cin.read();
    let (e, f): (i32, i32) = cin.read();

    //Read a vec of tuples. cnt is the number of tuples in the vector to read. Works with all types.
    let vec: Vec<(u32, u32)> = cin.read_vec(3);

    //Read a vec of non tuples. cnt is the number of elements to read. For non std types use impl_cin_type(type)! before.
    let vec1: Vec<u32> = cin.read_vec(3);

    println!("{} {} {} {} {} {}", a, b, c, d, e, f);

    for x in vec {
        println!("{} {}", x.0, x.1);
    }

    for x in vec1 {
        println!("{}", x);
    }
}
