use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
 
    let n_t = s.trim().parse::<usize>().unwrap();

    'outer: for _ in 0..n_t {
        stdin().read_line(&mut s).unwrap();
        s.clear();
        stdin().read_line(&mut s).unwrap();
        let vs: Vec<i32> = s.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        
        let mut n_1 = 0;

        let mut smallest = vs[0];

        for v in &vs{
            let v = *v;
            if v == 1 {
                n_1 += 1;
            }
            if v < smallest {
                smallest = v;
            }
        }
        if n_1 == 1 {
            println!("yEs");
            continue;
        }
        if n_1 > 1 {
            println!("nO");
            continue;
        }
        let mut n_smallest = 0;
        for v in &vs {
            if v % smallest != 0 {
                println!("yES");
                continue 'outer;
            }
            if *v == smallest {
                n_smallest += 1;
            }
        }
        if n_smallest == 1 {
            println!("Yes");
        }else{
            println!("No");
        }
    }

}