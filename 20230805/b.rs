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
    let mut tree = vec![-1; n];
    for _ in 0..m {
        input!((a,b): isize);
        let temp_parent = tree[(a - 1) as usize];
        let parent = if temp_parent != -1 {
            temp_parent
        } else {
            a - 1
        };
        tree[(b - 1) as usize] = parent;
        for i in 0..n {
            if tree[i] == b - 1 {
                tree[i] = parent;
            }
        }
    }
    let mut saikyo_cnt = 0;
    let mut saikyo = -1;
    for node in tree {
        if node == -1 {
            saikyo_cnt += 1;
            if saikyo_cnt > 1 {
                println!("-1");
                exit(0);
            }
            continue;
        }
        if saikyo == -1 {
            saikyo = node;
        } else if saikyo != node {
            println!("-1");
            exit(0);
        }
    }
    println!("{}", saikyo + 1);
}
