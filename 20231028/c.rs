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
    input!((_n, m): usize);
    let a_list = get_input::<usize>();
    let mut list: BTreeMap<usize, isize> = BTreeMap::new();
    for a in a_list {
        list.entry(a).and_modify(|e| *e += 1).or_insert(1);
        list.entry(a + m).and_modify(|e| *e -= 1).or_insert(-1);
    }
    let mut max = 0;
    let mut all_cnt = 0;
    for (_, cnt) in list {
        all_cnt += cnt;
        if all_cnt >= max {
            max = all_cnt;
        };
    }
    println!("{}", max);
}
