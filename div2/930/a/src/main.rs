use std::io::stdin;



fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t: usize = s.trim().parse().unwrap();

    for _ in 0..n_t {
        s.clear();
        
        stdin().read_line(&mut s).unwrap();

        let n: u32 = s.trim().parse().unwrap();

        let exp = (n as f32).log2().floor();

        let result = 2u32.pow(exp as u32) as u32;

        println!("{}", result);
    }
}
