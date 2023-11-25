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

fn main() {
    input!((d): f64);
    let max = d.sqrt().ceil() as usize;
    let min = (d / 2.0).sqrt().floor() as usize;
    let mut ans = max;
    for x in min..=max {
        let x_2 = x * x;
        let d = d as usize;
        if x_2 >= d {
            ans = ans.min(x_2 - d);
        } else {
            let rest = d - x_2;
            let y = (rest as f64).sqrt().floor() as usize;
            ans = ans.min(rest - y * y).min((y + 1) * (y + 1) - rest);
        }
    }
    println!("{}", ans);
}
