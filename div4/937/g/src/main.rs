macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn dfs(
    songs: &Vec<(usize, usize)>,
    genres: &Vec<Vec<usize>>,
    writers: &Vec<Vec<usize>>,
    used_songs: u32,
    curr_song: usize,
    cache: &mut Vec<i8>,
) -> bool {
    //println!("curr_song: {curr_song:b}, used_songs: {used_songs:b}");
    let cache_i = curr_song * ((1 << songs.len()) - 1) + used_songs as usize;
    if cache[cache_i] != 0 {
        if cache[cache_i] == -1 {
            return false;
        } else {
            return true;
        }
    }
    //println!("dfs: {used_songs:b}, curr: {curr_song:b}");
    if used_songs == (1 << songs.len()) - 1 {
        //println!("return");
        return true;
    } else {
        //println!("no return: {:b}, songs: {songs:?}", (2 << songs.len()) - 1);
    }
    let (genre, writer) = songs[curr_song];
    for song in &genres[genre] {
        let bit = 1 << (*song);
        if bit & used_songs == 0 {
            if dfs(songs, genres, writers, used_songs | bit, *song, cache) {
                cache[cache_i] = 1;
                return true;
            }
        }
    }
    for song in &writers[writer] {
        let bit = 1 << (*song);
        if bit & used_songs == 0 {
            if dfs(songs, genres, writers, used_songs | bit, *song, cache) {
                cache[cache_i] = 1;
                return true;
            }
        }
    }
    cache[cache_i] = -1;
    false
}
fn solve(s: &mut String) {
    let [n] = read_vals(s);

    let mut genres: HashMap<String, (usize, Vec<usize>)> = HashMap::new();
    let mut writers: HashMap<String, (usize, Vec<usize>)> = HashMap::new();
    let mut songs = vec![];
    for _ in 0..n {
        let [genre, writer] = read_vals_t(s, "".to_owned());
        let genre_n = if let Some((genre_n, others)) = genres.get_mut(&genre) {
            others.push(songs.len());
            *genre_n
        } else {
            genres.insert(genre, (genres.len(), vec![songs.len()]));
            genres.len() - 1
        };
        let writer_n = if let Some((writer_n, others)) = writers.get_mut(&writer) {
            others.push(songs.len());
            *writer_n
        } else {
            writers.insert(writer, (writers.len(), vec![songs.len()]));
            writers.len() - 1
        };
        songs.push((genre_n, writer_n));
    }
    let genres = {
        let mut g = vec![vec![]; genres.len()];
        for (_, (i, others)) in genres {
            g[i] = others;
        }
        g
    };
    let writers = {
        let mut w = vec![vec![]; writers.len()];
        for (_, (i, others)) in writers {
            w[i] = others;
        }
        w
    };
    let mut cache = vec![0i8; songs.len() * (1 << songs.len())];
    let mut min = u32::MAX;
    for start_song in 0..songs.len() {
        let bit = 1 << start_song;
        //println!("start_song: {start_song}, bit: {bit:b}");
        for used_songs in 0..((1 << songs.len()) - 1) {
            //println!("used_songs: {used_songs:b}");
            if used_songs & bit == 0 {
                if dfs(
                    &songs,
                    &genres,
                    &writers,
                    used_songs | bit,
                    start_song,
                    &mut cache,
                ) {
                    //println!("found");
                    min = min.min(used_songs.count_ones());
                }
            }
        }
    }
    result!("{min}");
}
fn main() {
    let mut s = String::new();

    let [n_t] = read_vals(&mut s);

    for _ in 0..n_t {
        solve(&mut s);
    }
}
use std::{
    collections::{HashMap, HashSet},
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
