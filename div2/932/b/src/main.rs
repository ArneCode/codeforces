use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t = s.trim().parse().unwrap();

    for _ in 0..n_t {
        stdin().read_line(&mut s).unwrap();
        s.clear();
        stdin().read_line(&mut s).unwrap();

        let arr: Vec<usize> = s
            .trim()
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let mut ns = vec![0u64; arr.len() + 1];

        let mut l = vec![arr.len() + 3; arr.len() + 1];
        let mut r = vec![0; arr.len() + 1];

        for (i, v) in arr.iter().enumerate() {
            ns[*v] += 1;

            l[*v] = l[*v].min(i);
            r[*v] = r[*v].max(i);
        }


        let mut max = 0;
        let mut max_n = 0;

        let mut max_l = 0;
        let mut min_r = usize::MAX;

        for (i, n) in ns.iter().enumerate() {
            if *n == 0 {
                break;
            }
            max = i + 1;
            max_n = *n;

            max_l = max_l.max(l[i]);
            min_r = min_r.min(r[i]);
        }
        //println!("max_l: {max_l}, min_r: {min_r}");

        if max == 0 && max_n == 0 {
            println!("2");
            println!("1 1");
            println!("2 {}", arr.len());
            continue;
        }

        if max_n == 1 || max_l >= min_r {
            println!("-1");
            continue;
        }

        println!("2");
        println!("1 {}", max_l + 1);
        println!("{} {}", max_l + 2, arr.len());
        // max -= 1;
        // for (i, v) in arr.iter().enumerate() {
        //     if *v == max {
        //         println!("1 {}", i + 1);
        //         println!("{} {}", i + 2, arr.len());
        //         break;
        //     }
        // }
    }
}
