use std::io::stdin;


fn r(a: &[u8], b: &[u8]) -> (Vec<u8>, usize) {
    //println!("r called with: B{:?}, A{:?}", a, b);
    let mut end: Option<(u8,u8, usize)> = None;
        for i in 0..(a.len() - 1) {
            let ea = a[i + 1];
            let eb = b[i];
            if ea == eb {
                continue;
            }
            end = Some((ea, eb, i));
            break;
    }
    
    return if let Some((ea, eb, end_i)) = end {
        let mut path = vec![a[0]];
        for i in 0..end_i {
            path.push(b[i]);
        }
        if ea > eb {
            for i in end_i..b.len() {
                path.push(b[i]);
            }
            //println!("returning {end_i}");
            (path, end_i + 1)
        }else{
            //path.push(a[end_i]);
            let (after_path, n) = r(&a[(end_i + 1)..], &b[(end_i + 1)..]);
            //println!("after_path: {:?}, n: {}", after_path, n);
            path.extend(after_path.iter());
            (path, n)
        }
    }else{
        let mut path = vec![a[0]];
        for i in 0..a.len() {
            path.push(b[i]);
        }
        (path, a.len())
    };
}
fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t: usize = s.trim().parse().unwrap();

    for _ in 0..n_t {
        s.clear();
        
        stdin().read_line(&mut s).unwrap();

        s.clear();

        stdin().read_line(&mut s).unwrap();

        let a = s.trim().chars().map(|c| {
            if c == '0' {
                0
            } else if c == '1' {
                1
            }else{
                println!("got: {}", c);
                panic!()
            }
        }).collect::<Vec<u8>>();

        s.clear();

        stdin().read_line(&mut s).unwrap();

        let b = s.trim().chars().map(|c| {
            if c == '0' {
                0
            } else if c == '1' {
                1
            }else{
                panic!()
            }
        }).collect::<Vec<u8>>();

        let (path, n) = r(&a, &b);
        for v in path {
            print!("{}", v);
        }
        println!();
        println!("{}", n);
    }
}
