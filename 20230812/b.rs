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
    input!((n): usize);
    let mut bet_list: Vec<(usize, Vec<usize>, usize)> = Vec::new();
    for i in 0..n {
        input!((c): usize);
        let a_list = get_input::<usize>();
        bet_list.push((c, a_list, i + 1));
    }
    input!((x): usize);
    let mut min = 37;
    let ans = bet_list
        .iter()
        .filter(|&bet| {
            if bet.1.contains(&x) {
                if bet.0 <= min {
                    min = bet.0;
                }
                return true;
            } else {
                false
            }
        })
        .collect::<Vec<_>>();
    let ans = ans
        .iter()
        .filter(|&&bet| bet.0 == min)
        .map(|(_, _, i)| format!("{}", i.to_string()))
        .collect::<Vec<String>>();
    println!("{}", ans.len());
    let ans = ans.join(" ");
    println!("{}", ans);
}
