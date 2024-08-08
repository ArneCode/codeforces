use std::{collections::HashSet, io::stdin};

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t = s.trim().parse().unwrap();
    'outer: for _ in 0..n_t {
        s.clear();
        stdin().read_line(&mut s).unwrap();

        let arr = s.trim().split(" ").map(|n| n.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let n = arr[0];
        let k = arr[1];
        let m = arr[2];

        s.clear();
        stdin().read_line(&mut s).unwrap();

        let base_set = (0..k).map(|n| char::from_u32('a' as u32 + n).unwrap()).collect::<HashSet<char>>();

        //println!("base_set: {base_set:?}");
        let mut set = base_set.clone();

        let mut result_s = "".to_owned();
        for c in s.chars(){
            set.remove(&c);
            if set.len() == 0 {
                result_s.push(c);
                set = base_set.clone();

                if result_s.len() as u32 == n {
                    println!("yEs");
                    continue 'outer;
                }
            }
        }
        result_s.push(set.into_iter().next().unwrap());
        while result_s.len() < n.try_into().unwrap() {
            result_s.push('a');
        }
        println!("No");
        println!("{}", result_s);
    }
}
