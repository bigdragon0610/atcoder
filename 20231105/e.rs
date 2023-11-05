use std::fmt::Debug;
use std::str::FromStr;

fn get_input<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|b| b.parse().unwrap()).collect()
}

macro_rules! input {
    (($($x:ident),+) : $t:ty) => {
      let mut input = get_input::<$t>().into_iter();
      $(let $x = input.next().unwrap();)*
    };
}

// macro_rules! get_tuple {
//     ($($x:ident : $t:ty),+) => {
//         let mut buf = String::new();
//         std::io::stdin().read_line(&mut buf).unwrap();
//         let mut iter = buf.split_whitespace();
//         $(let $x = iter.next().unwrap().parse::<$t>().unwrap();)*
//     };
// }

use std::f64::MIN;

fn main() {
    input!((n): usize);

    let p = get_input::<f64>();

    let mut dp = vec![vec![0_f64; n]; n];
    dp[0][0] = p[0];
    for i in 1..n {
        for j in 0..=i {
            if j == 0 {
                dp[0][i] = dp[0][i - 1].max(p[i]);
            } else if i == j {
                dp[j][i] = dp[j - 1][i - 1] * 0.9 + p[i];
            } else {
                dp[j][i] = dp[j][i - 1].max(dp[j - 1][i - 1] * 0.9 + p[i]);
            }
        }
    }

    let mut ans = MIN;
    let mut deno = 0.0;
    for (i, row) in dp.iter().enumerate() {
        deno = deno * 0.9 + 1.0;
        let sub = 1200.0 / ((i + 1) as f64).sqrt();
        ans = ans.max(row.iter().max_by_key(|n| n.to_bits()).unwrap() / deno - sub);
    }

    println!("{}", ans);
}
