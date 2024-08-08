macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn get_len(segs: &Vec<(u128, u128)>) -> u128 {
    let mut prev = segs[0];

    let mut len = 0;

    for seg in segs {
        len += prev.0.abs_diff(seg.0) + prev.1.abs_diff(seg.1);
        prev = *seg;
    }
    len
}
fn output(segs: &Vec<(u128, u128)>) {
    println!("{}", segs.len() - 1);

    let mut prev = segs[0];
    for seg in segs.into_iter().skip(1) {
        println!("{} {} {} {}", prev.0, prev.1, seg.0, seg.1);
        prev = *seg;
    }
}
fn solve(s: &mut String) {
    let [x1, y1] = read_vals(s);
    let [x2, y2] = read_vals(s);
    let [x3, y3] = read_vals(s);

    let pts: Vec<(i128, i128)> = vec![(x1, y1), (x2, y2), (x3, y3)];

    let mut sort_x = pts.clone();
    sort_x.sort_by_key(|(x, y)| *x);

    let (x1, y1) = sort_x[0];
    let (x2, y2) = sort_x[1];
    let (x3, y3) = sort_x[2];

    let mut segs = vec![];

    if (y1 <= y2 && y2 <= y3) || (y3 <= y2 && y2 <= y1) {
        segs.push(((x2, y2), (x2, y1)));
        segs.push(((x2, y2), (x2, y3)));
    } else if (y1 <= y3 && y3 <= y2) || (y1 >= y3 && y3 >= y2) {
        segs.push(((x2, y2), (x2, y3)));
        segs.push(((x2, y3), (x2, y1)));
    } else if (y3 <= y1 && y1 <= y2) || (y3 >= y1 && y1 >= y2) {
        segs.push(((x2, y2), (x2, y1)));
        segs.push(((x2, y1), (x2, y3)));
    }

    segs.push(((x1, y1), (x2, y1)));
    segs.push(((x3, y3), (x2, y3)));

    println!("4");
    for seg in segs {
        println!("{} {} {} {}", seg.0.0, seg.0.1, seg.1.0, seg.1.1);
    }
}
fn main() {
    let mut s = String::new();

    solve(&mut s);
}
use std::{
    fmt::Debug,
    io::stdin,
    ops::{Add, Div, Sub},
    str::FromStr,
    vec,
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
