//! # readput
//! Fast and easy stdin input parsing for competitive programming in rust.
//!
//! # Usage
//! **Note:** The inputs can be on the same line, or not, it doesn't make a difference. They are separated by spaces or line breaks. Excess inputs are getting cached for the next ```read()``` call.
//!
//! # Create a Scanner
//! Create a new ```Scanner```. (Only ASCII support for now)
//! ```Rust
//! use readput::AsciiScanner;
//! use readput::Scanner;
//!
//! let mut cin = AsciiScanner::new();
//! ```
//!
//! # Read a single value
//! Read a single value. For non std types use ```impl_cin_type!(type)``` before or use ```cin.read_cust_s()```.
//! **Note:** They also have to impl. FromStr and Debug.
//! ```Rust
//! let v: i128 = cin.read();
//! ```
//!
//! # Read a tuple
//! Read a tuple with variable size and custom types. (Works with all types that impl. ```FromStr``` and ```Debug```)
//! ```Rust
//! let (a, b, c): (String, i128, u32) = cin.read();
//! let (d, e): (i32, i32) = cin.read();
//! ```
//!
//! # Read a vector of tuples
//! Read a vector of tuples. 3 is the number of tuples in the vector to read. (Works with all types that impl. ```FromStr``` and ```Debug```)
//! ```Rust
//! let vec: Vec<(u32, String)> = cin.read_vec(3);
//! ```
//!
//! # Read a vector of single values
//! Read a vector of single values. 3 is the number of elements to read. For non std types use ```impl_cin_type!(type)``` before or use ```cin.read_cust_v()```.
//! **Note:** They also have to impl. FromStr and Debug.
//! ```Rust
//! let vec: Vec<u32> = cin.read_vec(3);
//! ```
//!
//! # Iterate over input
//! Iterate over input. This will iterate forever. (Blocks until new input is entered) For non std types use ```impl_cin_type!(type)``` before.
//! ```Rust
//! for (a, b) in cin.iter::<(String, u32)>() {
//!     println!("{} {}", a, b);
//! }
//! ```

use seq_macro::seq;
use std::{fmt::Debug, marker::PhantomData, str::FromStr};

macro_rules! impl_stdin_tuple {
    ($cnt:literal) => {
        seq! {N in 0..$cnt {
            impl<'a, #(T~N: FromStr,)*> Parseable<'a> for (#(T~N,)*) where
                #(T~N::Err: Debug,)*
            {
                type Ret = (#(T~N,)*);

                fn parse(sc: &mut impl Scanner<'a>) -> Self::Ret {
                    (#(sc.read_token().unwrap(),)*)
                }
            }
        }}
    };
}

macro_rules! impl_all_stdin_tuples {
    ($cnt:literal) => {
        seq! {N in 1..$cnt {
            #(impl_stdin_tuple!(N);)*
        }}
    };
}

#[macro_export]
macro_rules! impl_stdin_type {
    ($type:ty) => {
        impl<'a> Parseable<'a> for $type {
            type Ret = $type;
            fn parse(sc: &mut impl Scanner<'a>) -> Self::Ret {
                sc.read_token().unwrap()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_stdin_types {
    ($($type:ty),+) => {
        $(impl_stdin_type!($type);)*
    };
}

#[macro_export]
macro_rules! read {
    // Tuple
    ($sc:ident, ($($type:ty),+)) => {
        $sc.read::<($($type),+)>()
    };
    // Single value
    ($sc:ident, $type:ty) => {
        $sc.read::<($type,)>().0
    };
}

#[macro_export]
macro_rules! read_vec {
    ($sc:ident, $cnt:expr, ($($type:ty),+)) => {
        $sc.read_vec::<($($type),+)>($cnt)
    };
    ($sc:ident, $cnt:expr, $type:ty) => {
        $sc.read_vec::<($type,)>($cnt).into_iter().map(|x| x.0).collect()
    };
}

#[macro_export]
macro_rules! iter {
    ($sc:ident, $type:ty) => {
        $sc.iter::<($type,)>()
    };
}

pub trait Parseable<'a> {
    type Ret;

    fn parse(sc: &mut impl Scanner<'a>) -> Self::Ret;
}

impl_stdin_types!(
    u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, char, f32, f64, String
);

impl_all_stdin_tuples!(16);

pub trait Scanner<'a> {
    fn read_token<T: FromStr>(&mut self) -> Result<T, <T as FromStr>::Err>;

    fn read<T: Parseable<'a, Ret = T>>(&mut self) -> T;
    fn read_vec<T: Parseable<'a, Ret = T>>(&mut self, cnt: usize) -> Vec<T>;

    fn iter<T: Parseable<'a, Ret = T>>(&'a mut self) -> ScannerIter<'a, T, Self>
    where
        Self: Scanner<'a>,
        Self: std::marker::Sized,
    {
        ScannerIter {
            sc: self,
            phantom: PhantomData,
        }
    }
}

pub struct ScannerIter<'a, T: Parseable<'a, Ret = T>, S: Scanner<'a>> {
    sc: &'a mut S,
    phantom: PhantomData<T>,
}

impl<'a, T: Parseable<'a, Ret = T>, S: Scanner<'a>> ScannerIter<'a, T, S> {
    fn next(&mut self) -> T {
        self.sc.read::<T>()
    }
}

impl<'a, T: Parseable<'a, Ret = T>, S: Scanner<'a>> Iterator for ScannerIter<'a, T, S> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(ScannerIter::next(self))
    }
}

//Faster scanner that will only work if Ascii characters are typed into stdin.
#[derive(Default)]
pub struct AsciiScanner {
    buffer: String,
    ptr: usize,
}

impl AsciiScanner {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(20),
            ptr: 0,
        }
    }

    fn load_new_line(&mut self) {
        //Reset buffer and load new line.
        self.buffer.clear();
        if let Err(e) = std::io::stdin().read_line(&mut self.buffer) {
            panic!("{}", e);
        }
        self.ptr = 0;
    }
}

impl<'a> Scanner<'a> for AsciiScanner {
    fn read_token<T: FromStr>(&mut self) -> Result<T, <T as FromStr>::Err> {
        let mut found_token: i32 = -1;

        loop {
            if let Some(c) = self.buffer.as_bytes().get(self.ptr).copied() {
                match c as char {
                    ' ' => {
                        if found_token >= 0 {
                            return self.buffer[found_token as usize..self.ptr].parse();
                        } else {
                            self.ptr += 1;
                        }
                    }
                    '\n' | '\r' => {
                        if found_token >= 0 {
                            return self.buffer[found_token as usize..self.ptr].parse();
                        } else {
                            self.load_new_line();
                        }
                    }
                    _ => {
                        if found_token < 0 {
                            found_token = self.ptr as i32;
                        }
                        self.ptr += 1;
                    }
                }
            } else {
                self.load_new_line();
                continue;
            }
        }
    }

    fn read<T: Parseable<'a, Ret = T>>(&mut self) -> T {
        T::parse(self)
    }

    fn read_vec<T: Parseable<'a, Ret = T>>(&mut self, cnt: usize) -> Vec<T> {
        (0..cnt).map(|_| T::parse(self)).collect()
    }
}
