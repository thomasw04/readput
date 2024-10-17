use readput::iter;
use readput::read;
use readput::read_vec;
use readput::AsciiScanner;
use readput::Scanner;

fn main() {
    // In order to read types they must implement FromStr and Debug.
    let mut cin = AsciiScanner::new();

    //Read a single value.
    let a = read!(cin, i128);

    //Read a tuple with variable size.
    let (b, c, d) = read!(cin, (String, i128, u32));
    let (e, f) = read!(cin, (i32, i32));

    //Read a vec of tuples. 3 is the number of tuples to read.
    let vec = read_vec!(cin, 3, (u32, u32));

    for x in vec {
        println!("{} {}", x.0, x.1);
    }

    //Read a vec of non tuples.  3 is the number of elements to read.
    let vec: Vec<u32> = read_vec!(cin, 3, u32);

    for x in vec {
        println!("{}", x);
    }

    println!("{} {} {} {} {} {}", a, b, c, d, e, f);

    for (a, b) in iter!(cin, (String, u32)) {
        println!("{} {}", a, b);
    }
}
