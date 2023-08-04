use std::collections::HashSet;
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
    input!((n, _m): usize);
    let mut map: Vec<Vec<char>> = Vec::with_capacity(n);
    for _ in 0..n {
        input!((row): String);
        let row = row.chars().collect();
        map.push(row);
    }
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut stopped: HashSet<(usize, usize)> = HashSet::new();
    go((1, 1), (1, 0), &mut visited, &mut stopped, &map);
    go((1, 1), (0, 1), &mut visited, &mut stopped, &map);
    println!("{}", visited.len());
}

fn go(
    position: (usize, usize),
    direction: (isize, isize),
    visited: &mut HashSet<(usize, usize)>,
    stopped: &mut HashSet<(usize, usize)>,
    map: &Vec<Vec<char>>,
) {
    visited.insert(position);
    let next = (
        (position.0 as isize + direction.0) as usize,
        (position.1 as isize + direction.1) as usize,
    );
    if map[next.0][next.1] == '#' {
        if stopped.contains(&position) {
            return;
        }
        stopped.insert(position);
        if direction.0 == 0 {
            if map[position.0 + 1][position.1] != '#' {
                go((position.0 + 1, position.1), (1, 0), visited, stopped, map)
            }
            if map[position.0 - 1][position.1] != '#' {
                go((position.0 - 1, position.1), (-1, 0), visited, stopped, map)
            }
        }
        if direction.1 == 0 {
            if map[position.0][position.1 + 1] != '#' {
                go((position.0, position.1 + 1), (0, 1), visited, stopped, map)
            }
            if map[position.0][position.1 - 1] != '#' {
                go((position.0, position.1 - 1), (0, -1), visited, stopped, map)
            }
        }
    } else {
        go(next, direction, visited, stopped, map)
    }
}
