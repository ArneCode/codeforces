use std::{collections::{HashMap, HashSet}, io::stdin
};

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
    let mut divisors: HashMap<u128, Vec<usize>> = HashMap::new();

    for v in arr {
        if !divisors.contains_key(&v) {
            let mut divs = HashSet::new();
            divs.insert(1);
            let mut loop_v = v;
            let mut i = 2;
            while i < (loop_v as f32).sqrt().ceil() as u128 {
                if loop_v % i == 0 {
                    divs.insert(i as usize);
                    loop_v /= i;
                    if let Some(other_divs) = divisors.get(&loop_v) {
                        divs.extend(other_divs.iter());
                    }
                }
                i += 1;
            }
            divs.insert(v.try_into().expect("err_1"));
            let mut divs: Vec<usize> = divs.into_iter().collect();
            divs.sort();
            //println!("v: {v}, loop_v: {loop_v}");
            divisors.insert(v, divs);
        }
        
        let divs = divisors.get(&v).expect("err_2");
        for div in divs.iter().rev() {
            if a_arr.len() == *div {
                //println!("adding");
                a_arr.push(0);
            }else if a_arr.len() < *div {
                continue;
            }else{
                //println!("not adding, len: {}, div: {div}", a_arr.len());
            }
            a_arr[*div] += a_arr[*div - 1];
            a_arr[*div] %= MOD;
        }

        a_arr[1] += 1;
        //println!("added {v}, a_arr: {a_arr:?}, divs: {divs:?}");
    }

    let mut result: u128 = 0;
    //println!("arr: {a_arr:?}");
    for v in a_arr {
        result = (result + v) % MOD;
    }
    println!("{result}");
}
