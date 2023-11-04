use std::collections::HashSet;
use std::fmt::Debug;
use std::process::exit;
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

// macro_rules! input {
//     (($($x:ident),+) : $t:ty) => {
//       let mut input = get_input::<$t>().into_iter();
//       $(let $x = input.next().unwrap();)*
//     };
// }

// macro_rules! get_tuple {
//     ($($x:ident : $t:ty),+) => {
//         let mut buf = String::new();
//         std::io::stdin().read_line(&mut buf).unwrap();
//         let mut iter = buf.split_whitespace();
//         $(let $x = iter.next().unwrap().parse::<$t>().unwrap();)*
//     };
// }

fn main() {
    let mut map: Vec<Vec<u8>> = Vec::new();
    for _ in 0..9 {
        let row = get_input::<u8>();
        map.push(row);
    }
    let mut row: Vec<HashSet<u8>> = vec![HashSet::new(); 9];
    let mut col: Vec<HashSet<u8>> = vec![HashSet::new(); 9];
    let mut square: Vec<Vec<HashSet<u8>>> = vec![vec![HashSet::new(); 3]; 3];
    for i in 0..9 {
        for j in 0..9 {
            let cur = map[i][j];
            let ok_row = row[i].insert(cur);
            let ok_col = col[j].insert(cur);
            let ok_square = square[i / 3][j / 3].insert(cur);
            if !ok_row || !ok_col || !ok_square {
                println!("No");
                exit(0);
            }
        }
    }
    println!("Yes");
}
