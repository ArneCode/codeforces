use std::io::stdin;


fn n_inside(l: i32, a: i32) -> i32 {
    let mut n = 0;
    let mut l = l;

    while l % a == 0 {
        l = l/a;
        n += 1;
    }
    return n;
}
fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
 
    let n_t = s.trim().parse::<usize>().unwrap();

    for _ in 0..n_t {
        s.clear();
        stdin().read_line(&mut s).unwrap();
        let v: Vec<i32> = s.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        
        let a = v[1].min(v[0]);
        let b = v[0].max(v[1]);
        let l = v[2];

        if a == 1 && b == 1 {
            println!("1");
            continue;
        }
        if a == 1 {
            println!("{}", n_inside(l, b) + 1);
            continue;
        }
        if b == 1 {
            println!("{}", n_inside(l, a) + 1);
            continue;
        }

        let n_a = n_inside(l, a);
        let n_b = n_inside(l, b);
        //println!("{},{}, {}, n_a: {}", a, b, l, n_a);
        let a_in_b = n_inside(b, a);
        let do_thing = b == a.pow(a_in_b as u32);
        let mut sum = 0;
        // for n_div in 0..=n_a {
        //     let new_l = l / a.pow(n_div as u32);
        //     let b_inside = if do_thing{n_inside(new_l, b) + 1 - a_in_b*n_div} else{n_inside(new_l, b) + 1};
        //     //println!("b_inside: {}", b_inside);
        //     sum += b_inside;
        // }
        for n_div in 0..= n_b {
            let new_l = l / b.pow(n_div as u32);

            let a_inside = n_inside(new_l, a);

            if do_thing {
                sum += a_inside.min(a_in_b - 1) + 1;
            }else{
                sum += a_inside + 1;
            }
        }
        println!("{}",sum);
    }

}