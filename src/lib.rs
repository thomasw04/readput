use seq_macro::seq;
use std::{fmt::Debug, str::FromStr};

macro_rules! impl_stdin_tuple {
    ($cnt:literal) => {
        seq! {N in 0..$cnt {
            impl<#(T~N: FromStr,)*> Parseable for (#(T~N,)*) where
                #(T~N::Err: Debug,)*
            {
                type Ret = (#(T~N,)*);

                fn parse(sc: &mut impl Scanner) -> Self::Ret {
                    (#(sc.read_token().unwrap(),)*)
                }
            }
        }}
    };
}

macro_rules! impl_all_stdin_tuples {
    ($cnt:literal) => {
        seq! {N in 2..$cnt {
            #(impl_stdin_tuple!(N);)*
        }}
    };
}

macro_rules! impl_stdin_type {
    ($type:ty) => {
        impl Parseable for $type {
            type Ret = $type;
            fn parse(sc: &mut impl Scanner) -> Self::Ret {
                sc.read_token().unwrap()
            }
        }
    };
}

macro_rules! impl_stdin_types {
    ($($type:ty),+) => {
        $(impl_stdin_type!($type);)*
    };
}

pub trait Parseable {
    type Ret;

    fn parse(sc: &mut impl Scanner) -> Self::Ret;
}

impl_stdin_types!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, char, f32, f64, String
);

impl_all_stdin_tuples!(16);

pub trait Scanner {
    fn read_token<T: FromStr>(&mut self) -> Result<T, <T as FromStr>::Err>;

    fn read<T: Parseable<Ret = T>>(&mut self) -> T;
    fn read_cust_s<T: FromStr>(&mut self) -> T
    where
        T::Err: Debug;

    fn read_vec<T: Parseable<Ret = T>>(&mut self, cnt: usize) -> Vec<T>;
    fn read_cust_v<T: FromStr>(&mut self, cnt: usize) -> Vec<T>
    where
        T::Err: Debug;
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

impl Scanner for AsciiScanner {
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

    fn read<T: Parseable<Ret = T>>(&mut self) -> T {
        T::parse(self)
    }

    fn read_cust_s<T: FromStr>(&mut self) -> T
    where
        T::Err: Debug,
    {
        self.read_token().unwrap()
    }

    fn read_cust_v<T: FromStr>(&mut self, cnt: usize) -> Vec<T>
    where
        T::Err: Debug,
    {
        (0..cnt).map(|_| self.read_token::<T>().unwrap()).collect()
    }

    fn read_vec<T: Parseable<Ret = T>>(&mut self, cnt: usize) -> Vec<T> {
        (0..cnt).map(|_| T::parse(self)).collect()
    }
}
