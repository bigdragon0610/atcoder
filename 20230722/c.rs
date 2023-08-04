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
    input!((_n): usize);
    let dict = get_input::<usize>();
    let mut visited: Vec<usize> = Vec::new();
    let mut visited_hash: HashSet<usize> = HashSet::new();
    let mut next = 0;
    loop {
        if visited_hash.contains(&next) {
            let position = visited.iter().position(|v| *v == next).unwrap();
            let ans = visited[position..]
                .iter()
                .map(|n| (n + 1).to_string())
                .collect::<Vec<String>>();
            println!("{}", ans.len());
            println!("{}", ans.join(" "));
            break;
        } else {
            visited.push(next);
            visited_hash.insert(next);
            next = dict[next] - 1;
        }
    }
}
