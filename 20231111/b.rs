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
    input!((n): usize);
    let mut list: Vec<String> = Vec::new();
    for i in 1..=9 {
        let num_string = i.to_string();
        list.push(num_string.clone().repeat(2));
        list.push(num_string.clone().repeat(3));
        list.push(num_string.repeat(4));
    }

    let mut ans = 0;
    let d_list = get_input::<usize>();
    for i in 1..=n {
        for j in 1..=d_list[i - 1] {
            let num_string = i.to_string() + &j.to_string();
            if list.contains(&num_string) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
