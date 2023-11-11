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
    input!((s): String);
    let s = s.chars().collect::<Vec<char>>();
    let mut list: Vec<usize> = Vec::new();
    let mut sum = 0;
    list.push(0);
    for i in 1..n {
        if s[i - 1] == s[i] {
            sum += 1;
        }
        list.push(sum);
    }

    let mut ans = Vec::new();

    for _ in 0..q {
        input!((l, r): usize);
        ans.push(list[r - 1] - list[l - 1]);
    }

    println!(
        "{}",
        ans.iter().map(|n| n.to_string() + "\n").collect::<String>()
    )
}
