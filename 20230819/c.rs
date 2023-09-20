use std::collections::HashMap;
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
    let mut list: HashMap<usize, Vec<usize>> = HashMap::new();
    for _ in 0..n {
        input!((f, s): usize);
        list.entry(f)
            .and_modify(|e| {
                if e.len() == 2 {
                    let max = *e.iter().max().unwrap();
                    let min = *e.iter().min().unwrap();
                    if s > max {
                        *e = [s, max / 2].to_vec();
                    } else if s / 2 > min {
                        *e = [max, s / 2].to_vec();
                    }
                } else if s > e[0] {
                    *e = [s, e[0] / 2].to_vec();
                } else {
                    e.push(s / 2);
                }
            })
            .or_insert([s].to_vec());
    }
    let mut list = list
        .iter()
        .flat_map(|e| e.1)
        .copied()
        .collect::<Vec<usize>>();
    list.sort_by(|a, b| b.cmp(a));
    println!("{}", list[0] + list[1]);
}
