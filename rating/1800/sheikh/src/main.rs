macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn solve(inp: &mut InputReader<Stdin>, out: &mut OutputWriter<Stdout>) {
    // let [n, q] = read_vals_t(s, 0usize);
    let n: usize = inp.next();
    let q: usize = inp.next();
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        let elt: u64 = inp.next();
        arr.push(elt);
    }
    // let arr: Vec<u64> = read_arr(s);

    let mut prev_poses = Vec::with_capacity(arr.len());
    let mut prev_pos = None;
    for (i, v) in arr.iter().enumerate() {
        prev_poses.push(prev_pos);
        if *v > 0 {
            prev_pos = Some(i);
        }
    }
    let mut next_poses = Vec::with_capacity(arr.len());
    let mut next_pos = None;
    for (i, v) in arr.iter().enumerate().rev() {
        next_poses.push(next_pos);
        if *v > 0 {
            next_pos = Some(i);
        }
    }
    next_poses.reverse();
    // println!("next_poses: {next_poses:?}");

    let mut xors = vec![0];
    let mut xor = 0;
    for v in &arr {
        xor ^= v;
        xors.push(xor);
    }

    'q_loop: for _ in 0..q {
        //let [l_input, r_input] = read_vals_t(s, 0usize);
        let l_input: usize = inp.next();
        let r_input: usize = inp.next();
        let l_trans = l_input - 1;
        let r_trans = r_input - 1;

        let mut xor = xors[l_trans] ^ xors[r_trans + 1];
        let mut l = l_trans;

        let mut min = (r_trans + 1) - l_trans;
        let mut min_l = l_trans;
        let mut min_r = r_trans;
        loop {
            // println!("l = {l}, arr_elt {:b}, xor: {xor:b}", arr[l]);
            let mut r = r_trans;

            let mut loop_xor = xor;
            loop {
                // println!("r = {r}, arr_elt: {:b}, loop_xor: {loop_xor:b}", arr[r]);
                let len = (r + 1) - l;
                if len < min {
                    min_l = l;
                    min_r = r;
                    min = len;
                }
                if loop_xor & arr[r] == arr[r] {
                    if let Some(new_r) = prev_poses[r] {
                        if new_r <= l {
                            writeln!(out, "{l_input} {l_input}").unwrap();
                            continue 'q_loop;
                        }
                        loop_xor ^= arr[r];
                        r = new_r;
                    } else {
                        writeln!(out, "{l_input} {l_input}").unwrap();
                        continue 'q_loop;
                    }
                } else {
                    // println!("breaking r loop");
                    break;
                }
            }
            if xor & arr[l] == arr[l] {
                if let Some(new_l) = next_poses[l] {
                    if new_l >= r_trans {
                        writeln!(out, "{l_input} {l_input}").unwrap();
                        continue 'q_loop;
                    }
                    xor ^= arr[l];
                    l = new_l;
                } else {
                    writeln!(out, "{l_input} {l_input}").unwrap();
                    continue 'q_loop;
                }
            } else {
                // println!("breaking l loop");
                break;
            }
        }
        writeln!(out, "{} {}", min_l + 1, min_r + 1).unwrap();
    }
}
fn main() {
    let mut input = InputReader::new();
    let mut output = OutputWriter::new();

    let n_t: i32 = input.next();

    for _ in 0..n_t {
        solve(&mut input, &mut output);
    }
}
use std::{
    fmt::Debug,
    io::stdin,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, RemAssign, Sub},
    str::FromStr,
};
/// Returns the x of the first true Value
fn binary_edge<F, X>(f: F, min_x: X, max_x: X) -> X
where
    F: Fn(X) -> bool,
    X: PartialOrd
        + PartialEq
        + Add<Output = X>
        + Sub<Output = X>
        + Div<Output = X>
        + Copy
        + From<u8>,
{
    if f(min_x) {
        return min_x;
    }
    let mut a = min_x;
    let mut b = max_x;
    loop {
        let mid = (a + b) / 2.into();
        let mv = f(mid);

        if mv {
            b = mid;
        } else {
            a = mid;
        }

        if b - a == 1.into() {
            break b;
        }
    }
}

fn read_vals_t<T, const N: usize>(s: &mut String, elt: T) -> [T; N]
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    T: Debug,
{
    read_vals::<N, T>(s)
}

#[allow(invalid_type_param_default)]
fn read_vals<const N: usize, T = i128>(s: &mut String) -> [T; N]
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    T: Debug,
{
    read_arr(s).try_into().unwrap()
}
fn read_arr<T>(s: &mut String) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    read_line(s);
    s.trim()
        .split(" ")
        .map(|v| v.parse::<T>().unwrap())
        .collect()
}
fn read_line(s: &mut String) {
    s.clear();
    stdin().read_line(s).unwrap();
}
fn pow_mod<T>(mut b: T, mut e: T, mod_n: T) -> T
where
    T: From<u64>
        + PartialOrd
        + Rem<Output = T>
        + RemAssign
        + Eq
        + MulAssign
        + std::ops::ShrAssign<i32>
        + Copy,
{
    let mut r = T::from(1);
    while e > 0u64.into() {
        if e % 2u64.into() == 1u64.into() {
            r *= b;
            r %= mod_n;
        }
        b *= b;
        b %= mod_n;
        e >>= 1;
    }
    r
}
fn div_mod<T>(zähler: T, nenner: T, mod_n: T) -> T
where
    T: From<u64>
        + PartialOrd
        + Rem<Output = T>
        + RemAssign
        + Eq
        + MulAssign
        + std::ops::ShrAssign<i32>
        + Copy
        + Mul<Output = T>
        + Sub<Output = T>,
{
    (zähler * pow_mod(nenner, mod_n - 2u64.into(), mod_n)) % mod_n
}
fn fac_mod<T>(n: T, mod_n: T) -> T
where
    T: From<u64>
        + PartialOrd
        + Rem<Output = T>
        + RemAssign
        + Eq
        + MulAssign
        + std::ops::ShrAssign<i32>
        + Copy
        + Mul<Output = T>
        + Sub<Output = T>
        + AddAssign,
{
    let mut r = 1u64.into();
    let mut i = 1u64.into();
    while i <= n {
        r *= i;
        r %= mod_n;
        i += 1u64.into();
    }
    r
}
fn binomial_koef_mod<T>(n: T, k: T, mod_n: T) -> T
where
    T: From<u64>
        + PartialOrd
        + Rem<Output = T>
        + RemAssign
        + Eq
        + MulAssign
        + std::ops::ShrAssign<i32>
        + Copy
        + Mul<Output = T>
        + Sub<Output = T>
        + AddAssign,
{
    div_mod(
        fac_mod(n, mod_n),
        (fac_mod(n - k, mod_n) * fac_mod(k, mod_n)) % mod_n,
        mod_n,
    )
}
/*
  A fast and dead-simple reader for competitive programming in Rust

  Author: Axel Lindeberg, github.com/AxlLind
  Repository: https://github.com/AxlLind/easy_io
  License: MIT
  2020
*/
use std::fs::File;
#[allow(dead_code)]
use std::io::{self, Read, Stdin};

const EOF: &str = "InputReader: Reached end of input!";

pub struct InputReader<R: Read> {
    reader: R,
    buf: Vec<u8>,
    bytes_read: usize,
    current_index: usize,
}

impl InputReader<Stdin> {
    pub fn new() -> Self {
        Self::from_reader(io::stdin())
    }
}

impl InputReader<File> {
    pub fn from_file(path: &str) -> Self {
        Self::from_reader(File::open(path).unwrap())
    }
}

impl<R: Read> InputReader<R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            reader,
            buf: vec![0; 1 << 16],
            bytes_read: 0,
            current_index: 0,
        }
    }

    pub fn next<T: InputReadable>(&mut self) -> T {
        T::from_input(self)
    }

    pub fn next_line(&mut self) -> String {
        assert!(self.has_more(), "{EOF}");
        let mut line = String::new();
        while self.peek() != '\n' {
            line.push(self.peek());
            self.consume();
            if !self.has_more() {
                break;
            }
        }
        self.consume(); // consume '\n'
        line
    }

    pub fn has_more(&mut self) -> bool {
        if self.current_index >= self.bytes_read {
            self.bytes_read = self.reader.read(&mut self.buf[..]).unwrap();
            self.current_index = 0
        }
        self.bytes_read > 0
    }

    pub fn set_buf_size(&mut self, buf_size: usize) {
        self.buf.resize(buf_size, 0);
    }

    fn peek(&self) -> char {
        self.buf[self.current_index] as char
    }

    fn consume(&mut self) {
        self.current_index += 1;
    }

    fn pop_digit(&mut self) -> u64 {
        let c = self.peek();
        self.consume();
        c as u64 - '0' as u64
    }

    fn consume_until<F: Fn(char) -> bool>(&mut self, test: F) {
        loop {
            assert!(self.has_more(), "{EOF}");
            if test(self.peek()) {
                return;
            }
            self.consume();
        }
    }

    fn consume_until_sign(&mut self) -> i64 {
        loop {
            self.consume_until(|c| c.is_ascii_digit() || c == '-');
            if self.peek() != '-' {
                return 1;
            }

            self.consume();
            assert!(self.has_more(), "{EOF}");
            if self.peek().is_ascii_digit() {
                return -1;
            }
        }
    }
}

pub trait InputReadable {
    fn from_input<R: Read>(input: &mut InputReader<R>) -> Self;
}

impl InputReadable for u64 {
    fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
        input.consume_until(|c| c.is_ascii_digit());
        let mut num = 0;
        while input.peek().is_ascii_digit() {
            num = num * 10 + input.pop_digit();
            if !input.has_more() {
                break;
            }
        }
        num
    }
}

impl InputReadable for i64 {
    fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
        let sign = input.consume_until_sign();
        u64::from_input(input) as i64 * sign
    }
}

impl InputReadable for f64 {
    fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
        let sign = input.consume_until_sign() as f64;
        let mut num = 0.0;
        while input.peek().is_ascii_digit() {
            num = num * 10.0 + input.pop_digit() as f64;
            if !input.has_more() {
                break;
            }
        }

        let mut factor = 1.0;
        if input.peek() == '.' {
            input.consume();
            while input.has_more() && input.peek().is_ascii_digit() {
                num = num * 10.0 + input.pop_digit() as f64;
                factor *= 10.0;
            }
        }
        sign * num / factor
    }
}

impl InputReadable for String {
    fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
        input.consume_until(|c| c.is_ascii_graphic());
        let mut word = String::new();
        while input.peek().is_ascii_graphic() {
            word.push(input.peek());
            input.consume();
            if !input.has_more() {
                break;
            }
        }
        word
    }
}

impl InputReadable for char {
    fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
        input.consume_until(|c| c.is_ascii_graphic());
        let c = input.peek();
        input.consume();
        c
    }
}

macro_rules! impl_readable_from {
  ($A:ty, [$($T:ty),+]) => {
    $(impl InputReadable for $T {
      fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
        <$A>::from_input(input) as $T
      }
    })+
  };
}
impl_readable_from! { u64, [u32, u16, u8, usize] }
impl_readable_from! { i64, [i32, i16, i8, isize] }
impl_readable_from! { f64, [f32] }

/*
  A fast and dead-simple writer for competitive programming in Rust

  Author: Axel Lindeberg, github.com/AxlLind
  Repository: https://github.com/AxlLind/easy_io
  License: MIT
  2020
*/
use std::fmt::Display;
use std::fs::OpenOptions;
#[allow(dead_code)]
use std::io::{Result, Stdout, Write};

pub struct OutputWriter<W: Write> {
    writer: W,
    buf: Vec<u8>,
}

impl OutputWriter<Stdout> {
    pub fn new() -> Self {
        Self::from_writer(io::stdout())
    }
}

impl OutputWriter<File> {
    pub fn from_file(path: &str) -> Self {
        let file = OpenOptions::new().write(true).create(true).open(path);
        Self::from_writer(file.unwrap())
    }
}

impl<W: Write> OutputWriter<W> {
    pub fn from_writer(writer: W) -> Self {
        let buf = Vec::with_capacity(1 << 16);
        Self { writer, buf }
    }

    pub fn print<T: Display>(&mut self, t: T) {
        write!(self, "{}", t).unwrap();
    }

    pub fn prints<T: Display>(&mut self, t: T) {
        write!(self, "{} ", t).unwrap();
    }

    pub fn println<T: Display>(&mut self, t: T) {
        writeln!(self, "{}", t).unwrap();
    }
}

impl<W: Write> Write for OutputWriter<W> {
    fn write(&mut self, bytes: &[u8]) -> Result<usize> {
        self.buf.extend(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.write_all(&self.buf)?;
        self.writer.flush()?;
        self.buf.clear();
        Ok(())
    }
}

impl<W: Write> Drop for OutputWriter<W> {
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}
