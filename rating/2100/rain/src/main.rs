macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}

fn solve(inp: &mut InputReader<Stdin>) {
    //let [n, m] = read_vals_t(s, 0i64);
    let n: usize = inp.next();
    let m: i64 = inp.next();

    let mut rains = vec![];

    for _ in 0..n {
        //let [x, p] = read_vals_t(s, 0i64);
        let x: i64 = inp.next();
        let p: i64 = inp.next();
        rains.push((x,p));
    }

    let rains_copy = rains.clone();

    rains.sort_by_key(|(x, _)| *x);

    let mut starts = vec![];

    let mut mids = vec![];

    let mut ends = vec![];


    for (x, p) in &rains {
        let x = *x;
        let p = *p;

        starts.push(x - p);
        mids.push(x);
        ends.push(x + p);
    }

    starts.sort();

    ends.sort();

    let mut si = 0;
    let mut sm = 0;
    let mut se = 0;

    let mut npos: i64 = 0;
    let mut add = 0;

    let mut first_start = i64::MAX;
    let mut last_end = i64::MIN;

    for (x, _) in &rains {
        while si < starts.len() && starts[si] <= *x {
            npos += 1;
            add -= starts[si];
            si += 1;
        }
        while sm < mids.len() && mids[sm] <= *x {
            npos -= 2;
            add += 2*mids[sm];
            sm += 1;
        }
        while se < ends.len() && ends[se] <= *x {
            npos += 1;
            add -= ends[se];
            se += 1;
        }

        let v = x*npos + add;

        if v > m {
            let over = v - m;
            let start = x - over;
            let end = x + over;

            first_start = first_start.min(start);
            last_end = last_end.max(end);
        }  
    }
    let out = &mut BufWriter::new(stdout());
    for (x, p) in rains_copy {
        let start = x - p;
        let end = x + p;

        if first_start < start {
            write!(out, "0").unwrap();
            continue;
        }
        if last_end > end {
            write!(out, "0").unwrap();
            continue;
        }
        write!(out, "1").unwrap();
    }
    writeln!(out).unwrap();
}
fn main() {
    let mut input: InputReader<Stdin> = InputReader::new();

    let n_t: i32 = input.next();

    for _ in 0..n_t {
        solve(&mut input);
    }
}
use std::{
    fmt::Debug, i64, io::{stdin, stdout, BufWriter, Write}, ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, RemAssign, Sub}, str::FromStr
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
use std::io::{self, Read, Stdin};
use std::fs::File;


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
    assert!(self.has_more(), "EOF");
    let mut line = String::new();
    while self.peek() != '\n' {
      line.push(self.peek());
      self.consume();
      if !self.has_more() { break; }
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

  fn peek(&self) -> char { self.buf[self.current_index] as char }

  fn consume(&mut self) { self.current_index += 1; }

  fn pop_digit(&mut self) -> u64 {
    let c = self.peek();
    self.consume();
    c as u64 - '0' as u64
  }

  fn consume_until<F: Fn(char) -> bool>(&mut self, test: F) {
    loop {
      assert!(self.has_more(), "EOF");
      if test(self.peek()) { return; }
      self.consume();
    }
  }

  fn consume_until_sign(&mut self) -> i64 {
    loop {
      self.consume_until(|c| c.is_ascii_digit() || c == '-');
      if self.peek() != '-' { return 1; }

      self.consume();
      assert!(self.has_more(), "EOF");
      if self.peek().is_ascii_digit() { return -1; }
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
      if !input.has_more() { break; }
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
      if !input.has_more() { break; }
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
      if !input.has_more() { break; }
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
impl_readable_from!{ u64, [u32, u16, u8, usize] }
impl_readable_from!{ i64, [i32, i16, i8, isize] }
impl_readable_from!{ f64, [f32] }