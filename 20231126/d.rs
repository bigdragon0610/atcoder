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
    let mut map = Vec::new();
    for _ in 0..n {
        input!((s): String);
        map.push(s.chars().collect::<Vec<char>>())
    }
    let mut rows: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut cols: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let mut row = Vec::new();
        for j in 0..n {
            if map[i][j] == 'o' {
                row.push(j)
            }
        }
        rows.insert(i, row);
    }
    for i in 0..n {
        let mut col = Vec::new();
        for j in 0..n {
            if map[j][i] == 'o' {
                col.push(j)
            }
        }
        cols.insert(i, col);
    }
    let mut ans = 0;
    for (_i, row) in rows {
        if row.len() <= 1 {
            continue;
        }
        let mut sum = 0;
        for j in &row {
            match cols.get(j) {
                Some(col) => {
                    if col.len() >= 1 {
                        sum += col.len() - 1;
                    }
                }
                None => continue,
            }
        }
        ans += sum * (row.len() - 1);
    }
    println!("{}", ans);
}
