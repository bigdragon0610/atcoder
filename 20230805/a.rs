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
    let p_list = get_input::<usize>();
    let mut p_list = p_list.iter();
    let first = p_list.next().unwrap();
    let mut max = first;
    let mut duplication = false;
    while let Some(p) = p_list.next() {
        if p == max {
            duplication = true;
        }
        max = p.max(max);
    }
    if max == first {
        if duplication {
            println!("{}", 1);
        } else {
            println!("{}", 0);
        }
    } else {
        println!("{}", max - first + 1);
    }
}
