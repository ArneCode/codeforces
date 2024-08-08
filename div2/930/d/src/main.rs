use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t: usize = s.trim().parse().unwrap();

    for _ in 0..n_t {
        s.clear();

        stdin().read_line(&mut s).unwrap();

        s.clear();

        stdin().read_line(&mut s).unwrap();

        let a = s
            .trim()
            .chars()
            .map(|c| {
                if c == '>' {
                    1
                } else if c == '<' {
                    0
                } else {
                    println!("got: {}", c);
                    panic!()
                }
            })
            .collect::<Vec<u8>>();

        let mut nr = 0;
        let mut sum_r = vec![0];
        let mut nl = 0;
        let mut sum_l = vec![0];

        let mut r_poses = vec![0];
        let mut l_poses = vec![0];

        for (i, v) in a.iter().enumerate() {
            nr += v;
            sum_r.push(nr);
            nl += v + 1 % 2;
            sum_l.push(nl);

            if *v == 0 {
                l_poses.push(i + 1);
            } else {
                r_poses.push(i + 1);
            }
        }

        let mut rpos_s = 0;
        let mut rpos_s_a = vec![0];
        let mut lpos_s = 0;
        let mut lpos_s_a = vec![0];

        for (i, v) in a.iter().enumerate() {
            if *v == 0 {
                lpos_s += i + 1;
            } else {
                rpos_s += i + 1;
            }
            rpos_s_a.push(rpos_s);
            lpos_s_a.push(lpos_s);
        }
        println!("l_poses: {l_poses:?}, r_poses: {r_poses:?}, lpos_s_a: {lpos_s_a:?}, rpos_s_a: {rpos_s_a:?}");
        for i in 1..=(a.len()) {
            let n_l = sum_l.last().unwrap() - sum_l[i];
            let n_r = sum_r[i - 1];

            let result = if n_r > n_l {
                println!("a");
                let n_diff = n_r - n_l;

                let pos_left: usize = r_poses[n_diff as usize];
                let pos_right = a.len();
                //(pos_left, pos_right)
                todo!()
            } else {
                println!("b");
                let pos_left = 0;
                let pos_right = l_poses[n_r as usize + a[i - 1] as usize];
                println!("n_l: {}, n_r: {}, pos_left: {}, pos_right: {}, a_i: {}, i: {i}, lpos_s_a: {lpos_s_a:?}", n_l, n_r, pos_left, pos_right, a[i - 1]);

                let result = (lpos_s_a[pos_right] - lpos_s_a[i]) - (rpos_s_a[pos_left] - rpos_s_a[0]);
                result + i.abs_diff(pos_right)
            };
            print!("{}", result);
        }
        println!();
    }
}
