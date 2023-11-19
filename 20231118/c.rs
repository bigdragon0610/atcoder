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
    input!((s): String);
    let mut s = s.chars().collect::<Vec<char>>();
    s.push(' ');
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut cnt = 1;
    for i in 0..n {
        if s[i] == s[i + 1] {
            cnt += 1;
        } else {
            map.entry(s[i])
                .and_modify(|e| *e = (*e).max(cnt))
                .or_insert(cnt);
            cnt = 1;
        }
    }
    println!("{}", map.iter().map(|(_k, v)| *v).sum::<usize>())
}
