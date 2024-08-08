use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
 
    let n_t = s.trim().parse::<usize>().unwrap();

    for _ in 0..n_t {
        s.clear();
        stdin().read_line(&mut s).unwrap();
        let n = s.trim().parse::<usize>().unwrap();
        s.clear();
        stdin().read_line(&mut s).unwrap();
        let a: Vec<i32> = s.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        assert_eq!(a.len(), n);

        let mut max = 0;

        for v in a.iter() {
            max += v.abs();
        }
        println!("{}", max);
    }
}
