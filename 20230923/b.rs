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
    input!((_n, x): usize);
    let a_list = get_input::<usize>();
    let min = a_list.iter().min().unwrap();
    let max = a_list.iter().max().unwrap();
    let sum: usize = a_list.iter().sum();

    if x <= sum - max {
        println!("0");
    } else if sum - max <= x && x <= sum - min {
        println!("{}", x - (sum - max - min));
    } else {
        println!("-1");
    }
}
