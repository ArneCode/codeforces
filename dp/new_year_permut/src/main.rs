use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n: u128 = s.trim().parse().unwrap();

    const MOD: u128 = 998244353;
    let mut result = 1;
    let mut fact = 1;
    for i in 1..=n {
        fact = (i*fact)%MOD;
        result = ((result - 1)*i + fact) % MOD;
    }
    println!("{result}")
}
