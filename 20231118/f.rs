use std::collections::HashSet;
use std::fmt::Debug;
use std::mem::take;
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
    input!((_n, q): usize);
    let c_list = get_input::<usize>();
    let mut map: Vec<HashSet<usize>> = c_list.iter().map(|n| HashSet::from([*n])).collect();
    let mut ans = Vec::new();
    for _ in 0..q {
        input!((a, b): usize);
        let a = a - 1;
        let b = b - 1;
        if map[a].len() > map[b].len() {
            map.swap(a, b);
        }
        let tmp = take(&mut map[a]);
        map[b].extend(tmp);
        ans.push(map[b].iter().len());
    }
    println!(
        "{}",
        ans.iter().map(|n| format!("{}\n", n)).collect::<String>()
    );
}
