use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
 
    let n_t = s.trim().parse::<usize>().unwrap();

    for _ in 0..n_t {
        s.clear();
        stdin().read_line(&mut s).unwrap();
        let n = s.trim().parse::<usize>().unwrap();
        s.clear();
        stdin().read_line(&mut s).unwrap();
        let a: Vec<i32> = s.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        

        let mut sum = 0;
        let mut has_one = false;
        for i in 0..n {
            let elt = a[i];
            if elt % 3 == 1 {
                has_one = true;
            }
            sum += elt;
        }
        let m = sum % 3;
        let r = if m == 0{
            0
        }else if m == 1 {
            if has_one {
                1
            }else{
                2
            }
        }else{
            1
        };
        println!("{}",r);
    }

}