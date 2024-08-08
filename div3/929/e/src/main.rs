use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    let n_t = s.trim().parse::<usize>().unwrap();

    for _ in 0..n_t {
        stdin().read_line(&mut s).unwrap();
        s.clear();
        stdin().read_line(&mut s).unwrap();
        let a_i: Vec<i32> = s
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        //println!("a_i: {:?}", a_i);
        let mut a_sums = vec![0];
        let mut a_sum = 0;
        for a in &a_i {
            a_sum += a;
            a_sums.push(a_sum);
        }
        s.clear();
        stdin().read_line(&mut s).unwrap();

        let q = s.trim().parse::<usize>().unwrap();

        fn find_perf(l: usize, r: usize, u: i32, a_sums: &Vec<i32>) -> i32 {
            //println!("call find_perf: l: {}, r: {}, u: {}, a_sums: {:?}", l, r, u, a_sums);
            let sect = a_sums[r] - a_sums[l - 1];

            let result = u * sect - sect * (sect - 1) / 2;
            //println!("result: {}, sect: {}", result, sect);
            result
        }

        for _ in 0..q {
            s.clear();
            stdin().read_line(&mut s).unwrap();
            let lu: Vec<usize> = s
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let l = lu[0];
            let u = lu[1] as i32;

            let mut ra = l;
            let mut rb = a_i.len();

            let result = loop {
                if ra == rb {
                    break ra;
                }
                let mid = (ra + rb) / 2;

                let v = find_perf(l, mid, u, &a_sums);
                let vp = find_perf(l, mid + 1, u, &a_sums);

                if vp < v {
                    rb = mid;
                } else if vp == v {
                    break mid;
                } else {
                    ra = mid;
                }
                if rb - ra == 1 {
                    let va = find_perf(l, ra, u, &a_sums);
                    let vb = find_perf(l, rb, u, &a_sums);
                    if va < vb {
                        break rb;
                    } else {
                        break ra;
                    }
                }
            };
            if result == 3 && false {
                println!("ra: {}, rb: {}", ra, rb);
                println!(
                    "v(ra) = {}, v(rb) = {}",
                    find_perf(l, ra, u, &a_sums),
                    find_perf(l, rb, u, &a_sums)
                );
            }
            print!("{} ", result);
        }
        println!("");
    }
}
