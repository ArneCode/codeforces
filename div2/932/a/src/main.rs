use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t = s.trim().parse().unwrap();

    for _ in 0..n_t {
        stdin().read_line(&mut s).unwrap();
        s.clear();
        stdin().read_line(&mut s).unwrap();

        let s = s.trim().to_string();

        let mut s_p = s.chars().rev().collect::<String>();
        s_p.push_str(&s);
        //s_p = s_p.chars().rev().collect();

        //println!("s: {s}, s_p: {s_p}");
        let mut result = 0;
        for (ca, cb) in s.chars().zip(s_p.chars()) {
            if ca < cb {
                result = 0;
                break;
            }else if cb < ca {
                result = 1;
                break;
            }
        }
        if result == 0 {
            println!("{s}");
        }else{
            println!("{s_p}");
        }
    }
}
