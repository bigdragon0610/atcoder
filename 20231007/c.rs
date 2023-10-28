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
    let a_list: Vec<usize> = get_input();
    let mut points: Vec<(usize, Vec<usize>)> = Vec::with_capacity(n);
    for i in 0..n {
        input!((s): String);
        let mut cnt = i + 1;
        let mut not_answered: Vec<usize> = Vec::new();
        for (j, c) in s.chars().enumerate() {
            if c == 'o' {
                cnt += a_list[j];
            } else {
                not_answered.push(a_list[j]);
            }
        }
        points.push((cnt, not_answered));
    }
    let max = points.iter().max().unwrap().0;
    let ans = points
        .iter()
        .map(|(mut point, not_answered)| {
            let mut cnt = 0;
            let mut not_answered = not_answered.clone();
            not_answered.sort();
            while point < max {
                let last = not_answered.pop().unwrap();
                point += last;
                cnt += 1;
            }
            format!("{}\n", cnt)
        })
        .collect::<String>();
    print!("{}", ans);
}
