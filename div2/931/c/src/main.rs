use std::io::{stdin, stdout, Write};

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t: usize = s.trim().parse().unwrap();

    for _ in 0..n_t {
        s.clear();
        while s.trim() == "" {
            s.clear();
            stdin().read_line(&mut s).unwrap();
        }

        let arr = s
            .trim()
            .split(" ")
            .map(|c| c.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        s.clear();
        let n = arr[0];
        let m = arr[1];

        println!("? 1 1");
        stdout().flush().unwrap();

        s.clear();
        while s.trim() == "" {
            s.clear();
            stdin().read_line(&mut s).unwrap();
        }

        let d1: i64 = s.trim().parse().unwrap();

        println!("? 1 {m}");
        stdout().flush().unwrap();

        s.clear();
        while s.trim() == "" {
            s.clear();
            stdin().read_line(&mut s).unwrap();
        }

        let d2: i64 = s.trim().parse().unwrap();

        println!("? {n} 1");
        stdout().flush().unwrap();

        s.clear();
        while s.trim() == "" {
            s.clear();
            stdin().read_line(&mut s).unwrap();
        }

        let d3: i64 = s.trim().parse().unwrap();

        let x12_2 = d1 - d2 + m as i64 + 1;
        let x13_2 = d1 + d3 - n as i64 + 3;

        if x12_2 % 2 == 0 && x12_2 > 0 && x12_2 <= (m * 2).try_into().unwrap() {
            let x = x12_2 / 2;
            let y = d1 - x + 2;

            if y > 0 && y <= n.try_into().unwrap() {
                println!("? {y} {x}");
                stdout().flush().unwrap();

                s.clear();
                while s.trim() == "" {
                    s.clear();
                    stdin().read_line(&mut s).unwrap();
                }

                let d: u64 = s.trim().parse().unwrap();

                if d == 0 {
                    println!("! {y} {x}");
                    stdout().flush().unwrap();
                    continue;
                }
            }
        }

        assert!(x13_2 % 2 == 0);
        assert!(x13_2 > 0);

        let x = x13_2 / 2;
        let y = d1 - x + 2;
        println!("! {y} {x}");
        stdout().flush().unwrap();
    }
}
