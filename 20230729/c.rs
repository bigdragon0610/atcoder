use std::collections::BTreeMap;
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
    input!((_n, _m): usize);
    let a_list: Vec<usize> = get_input();
    let b_list: Vec<usize> = get_input();
    let mut list: BTreeMap<usize, [usize; 2]> = BTreeMap::new();
    let b_cnt = b_list.iter().count();
    let mut a_sum = 0;
    let mut b_sum = 0;
    for a in a_list {
        list.entry(a).and_modify(|v| v[0] += 1).or_insert([1, 0]);
    }
    for b in b_list {
        list.entry(b).and_modify(|v| v[1] += 1).or_insert([0, 1]);
    }
    for list in list {
        if b_cnt - b_sum <= a_sum + list.1[0] {
            println!("{}", list.0);
            break;
        }
        if a_sum + list.1[0] >= b_cnt - b_sum - list.1[1] {
            println!("{}", list.0 + 1);
            break;
        }
        a_sum += list.1[0];
        b_sum += list.1[1];
    }
}
