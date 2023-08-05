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
    input!((_n): usize);
    let a_list: Vec<usize> = get_input();
    let avg = a_list.iter().sum::<usize>() as f64 / a_list.len() as f64;
    let floor = avg.floor() as usize;
    let ceil = floor + 1;
    let mut cnt_p = 0;
    let mut cnt_n = 0;
    for a in a_list {
        if a <= floor {
            cnt_p += floor - a;
        } else {
            cnt_n += a - ceil;
        }
    }
    println!("{}", cnt_p.max(cnt_n));
}
