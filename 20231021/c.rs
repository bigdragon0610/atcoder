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
    input!((h, w): usize);
    let mut map: Vec<Vec<bool>> = Vec::new();
    let padding = vec![false; w + 2];
    map.push(padding.clone());
    for _ in 0..h {
        input!((s): String);
        let s = format!(".{}.", s);
        let row = s.chars().map(|c| c == '#').collect::<Vec<_>>();
        map.push(row);
    }
    map.push(padding);

    let mut cnt = 0;
    for i in 1..h + 1 {
        for j in 1..w + 1 {
            if map[i][j] {
                cnt += 1;
                search(i, j, &mut map);
            }
        }
    }

    println!("{}", cnt);
}

fn search(i: usize, j: usize, map: &mut Vec<Vec<bool>>) {
    if !map[i][j] {
        return;
    }

    map[i][j] = false;

    let siblings = [
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ];
    for (next_i, next_j) in siblings {
        search(next_i, next_j, map);
    }
}
