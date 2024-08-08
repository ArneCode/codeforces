macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn fill(
    data: &Vec<bool>,
    n_rows: usize,
    n_cols: usize,
    x: usize,
    y: usize,
    v: u32,
    markers: &mut Vec<u32>,
) {
    let i = y * n_cols + x;
    if !data[i] {
        return;
    }
    if markers[i] > 0 {
        return;
    }
    markers[i] = v;
    for (dirx, diry) in vec![(0i64, 1i64), (0, -1), (1, 0), (-1, 0)] {
        let new_x = x as i64 + dirx;
        let new_y = y as i64 + diry;
        if new_x < 0 || new_y < 0 || new_x as usize >= n_cols || new_y as usize >= n_rows {
            continue;
        }
        fill(
            data,
            n_rows,
            n_cols,
            new_x as usize,
            new_y as usize,
            v,
            markers,
        );
    }
}
fn solve(s: &mut String) {
    let [n_rows, n_cols] = read_vals_t(s, 0usize);
    let mut data = vec![false; n_rows * n_cols];
    for y in 0..n_rows {
        //let arr: Vec<String> = read_arr(s);
        read_line(s);
        let s = s.trim();
        for (x, c) in s.chars().enumerate() {
            let i = y * n_cols + x;
            //println!("s: '{s}', x: {x}, y: {y}");
            data[i] = c == '#';
        }
    }
    let mut rows_covered = vec![false; n_rows];
    //println!("data: {data:?}");
    for y in 0..n_rows {
        let mut curr_true = false;
        let mut prev_true = false;
        for x in 0..n_cols {
            let i = y * n_cols + x;
            //print!("{} ", data[i]);
            if data[i] {
                if curr_true {
                    continue;
                } else if !prev_true {
                    curr_true = true;
                } else {
                    println!("-1");
                    return;
                }
            } else if curr_true {
                curr_true = false;
                prev_true = true;
            }
        }
        rows_covered[y] = curr_true || prev_true;
        //println!();
        // if !(curr_true || prev_true) {
        //     println!("-1");
        //     return;
        // }
    }
    let mut cols_covered = vec![false; n_cols];
    for x in 0..n_cols {
        let mut curr_true = false;
        let mut prev_true = false;
        for y in 0..n_rows {
            let i = y * n_cols + x;
            if data[i] {
                if curr_true {
                    continue;
                } else if !prev_true {
                    curr_true = true;
                } else {
                    println!("-1");
                    return;
                }
            } else if curr_true {
                curr_true = false;
                prev_true = true;
            }
        }
        cols_covered[x] = curr_true || prev_true;
        // if !(curr_true || prev_true) {
        //     println!("-1");
        //     return;
        // }
    }
    'outer: for y in 0..n_rows {
        if rows_covered[y] {
            continue;
        }
        for x in 0..n_cols {
            // let i = y*n_cols + x;
            if !cols_covered[x] {
                continue 'outer;
            }
        }
        println!("-1");
        return;
    }
    'outer: for x in 0..n_cols {
        if cols_covered[x] {
            continue;
        }
        for y in 0..n_rows {
            // let i = y*n_cols + x;
            if !rows_covered[y] {
                continue 'outer;
            }
        }
        println!("-1");
        return;
    }
    //println!("here");
    let mut n_neg = 0;
    let mut markers = vec![0; data.len()];
    for y in 0..n_rows {
        for x in 0..n_cols {
            let i = y * n_cols + x;
            if markers[i] > 0 || !data[i] {
                continue;
            }
            n_neg += 1;
            fill(&data, n_rows, n_cols, x, y, n_neg, &mut markers);
        }
    }
    result!("{n_neg}");
}
fn main() {
    let mut s = String::new();

    // let [n_t] = read_vals(&mut s);

    // for _ in 0..n_t {
    solve(&mut s);
    // }
}
use std::{
    fmt::Debug,
    io::stdin,
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
