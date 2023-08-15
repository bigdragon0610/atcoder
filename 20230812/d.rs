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

macro_rules! get_tuple {
    ($($x:ident : $t:ty),+) => {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        $(let $x = iter.next().unwrap().parse::<$t>().unwrap();)*
    };
}

fn main() {
    input!((_n): usize);
    input!((s): String);
    let mut s = s;
    input!((q): usize);
    let mut rep: HashMap<usize, (char, usize)> = HashMap::new();
    let mut rep_all = (0, 0);
    for i in 0..q {
        get_tuple!(t: usize, x: usize, c: char);
        if t == 1 {
            rep.insert(x, (c, i));
        } else {
            rep_all = (t, i);
        }
    }
    if rep_all.0 == 2 {
        s = s.to_lowercase();
    } else if rep_all.0 == 3 {
        s = s.to_uppercase();
    }
    let mut s = s.chars().collect::<Vec<char>>();
    for (position, (mut c, i)) in rep {
        if i > rep_all.1 {
            s[position - 1] = c;
        } else {
            if rep_all.0 == 2 {
                c = c.to_lowercase().next().unwrap();
            } else if rep_all.0 == 3 {
                c = c.to_uppercase().next().unwrap();
            }
            s[position - 1] = c;
        }
    }
    println!("{}", s.iter().map(|c| c.to_string()).collect::<String>());
}
