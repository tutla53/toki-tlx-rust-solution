#![allow(unused)]

use {
    std::io::stdin,
    std::str::FromStr,
    std::fmt::Debug,
    std::collections::BinaryHeap,
    std::cmp::Reverse
};

fn take_int<T: FromStr>() -> T where <T as FromStr>::Err: Debug{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap()
}

fn take_vector<T: FromStr>() -> Vec<T> where <T as FromStr>::Err: Debug{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let arr: Vec<T> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    return arr;
}

fn take_string() -> Vec<char> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let vec:Vec<char> = input.trim().chars().collect();
    return vec;
}

fn to_string(vec:Vec<char>) -> String{
    return vec.iter().collect::<String>();
}

fn main(){
    let mut heap = BinaryHeap::new();

    heap.push(Reverse(1));
    heap.push(Reverse(2));
    heap.push(Reverse(5));

    let mut m: u8;
    while(!heap.is_empty()){
        match heap.peek(){
            None => m = 0,
            Some(Reverse(value)) => {
                m = *value
            },
        }
        println!("{}", m);
        heap.pop();
    }
}