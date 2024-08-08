macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn section_off(sect_size: u32, verts: &Vec<Vec<usize>>, i: usize) -> (u32, u32) {
    // => (vert_size, n_cut)
    let mut size = 1;
    let mut n_cut = 0;
    for vert in &verts[i] {
        let (v_size, v_cut) = section_off(sect_size, verts, *vert);
        size += v_size;
        n_cut += v_cut;
    }
    if size >= sect_size && i != 0 {
        n_cut += 1;
        size = 0;
    }
    if i == 0 {
        if size < sect_size && n_cut >0{
            n_cut -= 1;
        }
        //println!("sect_size:  {sect_size}, n_cut: {n_cut}");
    }
    (size, n_cut)
}
fn solve(s: &mut String) {
    let [n, k] = read_vals_t(s, 0u128);

    let mut vert: Vec<Vec<usize>> = vec![vec![]; n as usize];
    for _ in 0..(n - 1) {
        let [mut v, mut u] = read_vals_t(s, 0usize);
        if v > u {
            mem::swap(&mut v, &mut u);
        }
        vert[v - 1].push(u - 1);
    }
    let max_r = n / k;
    //println!("vert: {vert:?}");
    // let result = binary_edge(|x| {
    //     section_off(x as u32, &vert, 0).1 <= k as u32
    // }, 0, max_r + 1) ;
    // for size in (0..=(max_r)).rev() {
    //     let (_, cuts) = section_off(size as u32, &vert, 0);
    //     if cuts >= k as u32 {
    //         result!("{size}");
    //         return;
    //     }
    // }
    let r = binary_edge(|x| {
        //println!("x, max_x: {}, {}", x, max_r);
        let x = ((max_r as i32 + 1) - x as i32);
        section_off(x as u32, &vert, 0).1 >= k as u32
    }, 0, max_r  + 1);
    let r = max_r + 1 - r;
    result!("{r}");
}
fn main() {
    let mut s = String::new();

    let [n_t] = read_vals(&mut s);

    for _ in 0..n_t {
        solve(&mut s);
    }
}
use std::{
    fmt::Debug,
    io::stdin,
    mem,
    ops::{Add, Div, Sub},
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
