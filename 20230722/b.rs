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
    input!((n, d): usize);
    let mut schedule = vec!['o'; d];
    for _ in 0..n {
        input!((s): String);
        for (i, c) in s.chars().enumerate() {
            if c == 'x' {
                schedule[i] = 'x';
            }
        }
    }
    let schedule = schedule.split(|c| *c == 'x');
    let mut max = 0;
    for day in schedule {
        let len = day.len();
        if max <= len {
            max = len
        }
    }
    println!("{}", max);
}
