use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t: usize = s.trim().parse().unwrap();

    for _ in 0..n_t {
        stdin().read_line(&mut s).unwrap();
        s.clear();
        stdin().read_line(&mut s).unwrap();

        let arr = s.trim().split(" ").map(|c| c.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        let mut max_i = 0;
        let mut max_v = i64::MIN;

        let mut max_2_i = 0;
        let mut max_2 = i64::MIN;

        let mut min_i = 0;
        let mut min_v = i64::MAX;

        let mut min_2_i = 0;
        let mut min_2 = i64::MAX;

        for (i,v) in arr.iter().enumerate() {
            if *v >= max_v {
                max_2 = max_v;

                max_v = *v;
                max_i = i;
            }else if *v >= max_2 {
                max_2 = *v;
            }
            if *v <= min_v {
                min_2 = min_v;
                min_v = *v;
                min_i = i;
            }else if *v <= min_2 {
                min_2 = *v;
            }
        }
        //println!("{max_v}, {max_2}, {min_v}, {min_2}");

        let result = (max_v - min_v).abs() + (max_2 - min_v).abs() + (max_2 - min_2).abs() + (max_v - min_2).abs();
        println!("{result}");
    }
}
