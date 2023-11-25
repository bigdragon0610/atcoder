use std::collections::BTreeSet;
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
    input!((n, q): usize);
    let mut a_list = get_input::<usize>();
    let mut list: Vec<usize> = vec![0; n + 1];
    for &a in &a_list {
        if a > n {
            continue;
        }
        list[a] += 1;
    }
    let mut zeros: BTreeSet<usize> = list
        .iter()
        .enumerate()
        .filter(|(_, n)| **n == 0)
        .map(|(i, _)| i)
        .collect();
    for _ in 0..q {
        input!((i, x): usize);
        let pre_a = a_list[i - 1];
        a_list[i - 1] = x;
        if pre_a <= n {
            list[pre_a] -= 1;
            if list[pre_a] == 0 {
                zeros.insert(pre_a);
            }
        }
        if x <= n {
            if list[x] == 0 {
                zeros.remove(&x);
            }
            list[x] += 1;
        }
        println!("{}", zeros.first().unwrap());
    }
}
