use std::{str::FromStr, fmt::Debug, ffi::OsString};

use readput_macros::impl_cin_type;

impl_cin_type!(u8);
impl_cin_type!(u16);
impl_cin_type!(u32);
impl_cin_type!(u64);
impl_cin_type!(u128);
impl_cin_type!(usize);

impl_cin_type!(i8);
impl_cin_type!(i16);
impl_cin_type!(i32);
impl_cin_type!(i64);
impl_cin_type!(i128);
impl_cin_type!(isize);

impl_cin_type!(bool);
impl_cin_type!(char);
impl_cin_type!(f32);
impl_cin_type!(f64);
impl_cin_type!(String);

pub trait Parseable {
    type Ret;

    fn parse(sc: &mut impl Scanner) -> Self::Ret;
}

impl<T0: FromStr, T1: FromStr> Parseable for (T0, T1) where 
    T0::Err: Debug,
    T1::Err: Debug
{
    type Ret = (T0, T1);

    fn parse(sc: &mut impl Scanner) -> Self::Ret {
        (sc.read_token().unwrap(), sc.read_token().unwrap())
    }
}

impl<T0: FromStr, T1: FromStr, T2: FromStr> Parseable for (T0, T1, T2) where 
    T0::Err: Debug,
    T1::Err: Debug,
    T2::Err: Debug
{
    type Ret = (T0, T1, T2);

    fn parse(sc: &mut impl Scanner) -> Self::Ret {
        (sc.read_token().unwrap(), sc.read_token().unwrap(), sc.read_token().unwrap())
    }
}

impl<T0: FromStr, T1: FromStr, T2: FromStr, T3: FromStr> Parseable for (T0, T1, T2, T3) where 
    T0::Err: Debug,
    T1::Err: Debug,
    T2::Err: Debug,
    T3::Err: Debug
{
    type Ret = (T0, T1, T2, T3);

    fn parse(sc: &mut impl Scanner) -> Self::Ret {
        (sc.read_token().unwrap(), sc.read_token().unwrap(), sc.read_token().unwrap(), sc.read_token().unwrap())
    }
}

impl<T0: FromStr, T1: FromStr, T2: FromStr, T3: FromStr, T4: FromStr> Parseable for (T0, T1, T2, T3, T4) where 
    T0::Err: Debug,
    T1::Err: Debug,
    T2::Err: Debug,
    T3::Err: Debug,
    T4::Err: Debug
{
    type Ret = (T0, T1, T2, T3, T4);

    fn parse(sc: &mut impl Scanner) -> Self::Ret {
        (sc.read_token().unwrap(), sc.read_token().unwrap(), sc.read_token().unwrap(), sc.read_token().unwrap(), sc.read_token().unwrap())
    }
}

impl<T0: FromStr, T1: FromStr, T2: FromStr, T3: FromStr, T4: FromStr, T5: FromStr> Parseable for (T0, T1, T2, T3, T4, T5) where 
    T0::Err: Debug,
    T1::Err: Debug,
    T2::Err: Debug,
    T3::Err: Debug,
    T4::Err: Debug,
    T5::Err: Debug
{
    type Ret = (T0, T1, T2, T3, T4, T5);

    fn parse(sc: &mut impl Scanner) -> Self::Ret {
        (sc.read_token().unwrap(), sc.read_token().unwrap(), sc.read_token().unwrap(), sc.read_token().unwrap(), sc.read_token().unwrap(), sc.read_token().unwrap())
    }
}

pub trait Scanner {
    fn read_token<T: FromStr>(&mut self) -> Result<T, <T as FromStr>::Err>;

    fn read<T: Parseable<Ret = T>>(&mut self) -> T;
    fn read_cust_s<T: FromStr>(&mut self) -> T where T::Err: Debug;

    fn read_vec<T: Parseable<Ret = T>>(&mut self, cnt: u32) -> Vec<T>;
    fn read_cust_v<T: FromStr>(&mut self, cnt: u32) -> Vec<T> where T::Err: Debug;
}

//Faster scanner that will only work if Ascii characters are typed into stdin. 
#[derive(Default)]
pub struct AsciiScanner
{
    buffer: String,
    ptr: usize,
}

impl AsciiScanner {
    pub fn new() -> Self {
        Self { buffer: String::with_capacity(20), ptr: 0 }
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

impl Scanner for AsciiScanner
{
    fn read_token<T: FromStr>(&mut self) -> Result<T, <T as FromStr>::Err>
    {
        let mut found_token: i32 = -1;

        loop {
            if let Some(c) = self.buffer.as_bytes().get(self.ptr).copied() {
                match c as char {
                    ' ' => { 
                        if found_token >= 0 {
                            return self.buffer[found_token as usize .. self.ptr].parse();
                        } else {
                            self.ptr += 1;
                        }
                    },
                    '\n' | '\r' => {
                        if found_token >= 0 {
                            return self.buffer[found_token as usize .. self.ptr].parse();
                        } else {
                            self.load_new_line();
                        }
                    },
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

    fn read_cust_s<T: FromStr>(&mut self) -> T where T::Err: Debug {
        self.read_token().unwrap()
    }

    fn read_cust_v<T: FromStr>(&mut self, cnt: u32) -> Vec<T> where T::Err: Debug {
        let mut v: Vec<T> = Vec::new();

        for _i in 0..cnt {
            v.push(self.read_token::<T>().unwrap());
        }

        v
    }

    fn read_vec<T: Parseable<Ret = T>>(&mut self, cnt: u32) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();

        for _i in 0..cnt {
            v.push(T::parse(self));
        }

        v
    }
}
