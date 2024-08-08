use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();
    s.clear();

    stdin().read_line(&mut s).unwrap();

    let arr: Vec<u128> = s
        .trim()
        .split(" ")
        .map(|c| c.parse::<u128>().unwrap())
        .collect();
    const MOD: u128 = 10_u128.pow(9) + 7;
    let mut a_arr = vec![0u128, 0u128];
    for v in arr {
        for i in (1..a_arr.len().min(1100)).rev() {
            if v as usize % (i + 1) == 0 {
                if a_arr.len() == i + 1 {
                    a_arr.push(a_arr[i]);
                }else{
                    a_arr[i+1] = (a_arr[i+1] + a_arr[i]) % MOD;
                }
            }
        }
        a_arr[1] += 1;
        //println!("added {v}, a_arr: {a_arr:?}");
    }
    let mut result: u128 = 0;
    //println!("arr: {a_arr:?}");
    for v in a_arr{
        result = (result + v) % MOD;
    }
    println!("{result}");
}
