use std::collections::VecDeque;
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
    input!((n, m): usize);
    input!((s): String);
    let c_list = get_input::<usize>();
    let mut q: Vec<VecDeque<char>> = vec![VecDeque::new(); m];
    for (char, c) in s.chars().zip(&c_list) {
        q[c - 1].push_back(char);
    }
    for row in &mut q {
        let front = row.pop_back().unwrap();
        row.push_front(front);
    }
    let mut ans: Vec<char> = Vec::with_capacity(n);
    for c in &c_list {
        let char = q[c - 1].pop_front().unwrap();
        ans.push(char);
    }
    let ans = ans.iter().collect::<String>();
    println!("{}", ans);
}
