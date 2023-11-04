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
    let a_list = get_input::<usize>();
    let b_list = get_input::<usize>();
    let mut tree: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for i in 0..m {
        let a = a_list[i];
        let b = b_list[i];
        if a == b {
            println!("No");
            exit(0);
        }
        tree[a - 1].insert(b);
        tree[b - 1].insert(a);
    }
    let mut ans: [HashSet<usize>; 2] = [HashSet::new(), HashSet::new()];
    let mut visited = vec![false; n];
    for a in a_list {
        search(&tree, &mut ans, a - 1, true, &mut visited);
    }
    println!("Yes");
}

fn search(
    tree: &Vec<HashSet<usize>>,
    ans: &mut [HashSet<usize>; 2],
    i: usize,
    is_left: bool,
    visited: &mut Vec<bool>,
) {
    if !visited[i] {
        visited[i] = true;
    } else {
        return;
    }
    let lr = if is_left { 0 } else { 1 };
    ans[lr].insert(i);
    for node in &tree[i] {
        let next_i = node - 1;
        if ans[lr].get(&next_i).is_some() {
            println!("No");
            exit(0);
        }
        search(tree, ans, next_i, !is_left, visited);
    }
}
