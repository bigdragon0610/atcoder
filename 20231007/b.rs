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
    let mut table: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        input!((s): String);
        let mut cnt = 0;
        for c in s.chars() {
            if c == 'o' {
                cnt += 1;
            }
        }
        table.push(cnt);
    }
    let mut table: Vec<(usize, &usize)> = table.iter().enumerate().collect();
    table.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            b.1.cmp(a.1)
        }
    });
    let ans = table
        .iter()
        .map(|val| format!("{} ", val.0 + 1))
        .collect::<String>();
    println!("{}", ans);
}
