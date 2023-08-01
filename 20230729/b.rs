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
    let mut field: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        input!((s): String);
        let s = s.chars().collect();
        field.push(s);
    }
    for n in 0..n - 8 {
        'm: for m in 0..m - 8 {
            for i in 0..3 {
                for j in 0..3 {
                    if field[n + i][m + j] != '#'
                        || field[n + i + 6][m + j + 6] != '#'
                        || field[n + i][m + 3] != '.'
                        || field[n + 3][m + j] != '.'
                        || field[n + i + 5][m + 5] != '.'
                        || field[n + 5][m + j + 5] != '.'
                    {
                        continue 'm;
                    }
                }
            }
            if field[n + 3][m + 3] == '.' && field[n + 5][m + 5] == '.' {
                println!("{} {}", n + 1, m + 1);
            }
        }
    }
}
