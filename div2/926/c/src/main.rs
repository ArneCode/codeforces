use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t = s.trim().parse().unwrap();

    'outer: for _ in 0..n_t {
        s.clear();
        stdin().read_line(&mut s).unwrap();

        let arr = s.trim().split(" ").map(|n| n.parse::<u128>().unwrap()).collect::<Vec<u128>>();

        let k = arr[0];
        let x = arr[1];
        let mut a = arr[2];

        let mut lost = 0;
        let mut bet = 1;
        for i in 0..=x {
            // while bet*(k) <= lost + bet {
            //     bet += 1;
            // }
            bet = ((lost as f64)/((k - 1) as f64)) as u128 + 1;
            // if i == x {
            //     while bet*(k) <= lost + bet {
            //         bet += 1;
            //     }
            // }
            if a < bet {
                println!("nO");
                continue 'outer;
            }
            //println!("bet: {bet}");
            a -= bet;
            lost += bet;
        }
        println!("YES");
    }
}
