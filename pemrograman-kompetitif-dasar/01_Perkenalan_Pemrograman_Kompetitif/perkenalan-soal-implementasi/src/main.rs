#![allow(unused)]

use {
    std::io::stdin,
    std::str::FromStr,
    std::fmt::Debug
};

fn take_vector<T: FromStr>() -> Vec<T> where <T as FromStr>::Err: Debug{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let arr: Vec<T> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    return arr;
}

fn main(){
    let mut arr: Vec<Vec<u8>> = vec![vec![0; 100]; 100];
    let mut rotate: Vec<Vec<u8>> = vec![vec![0; 100]; 100];

    let dimension: Vec<usize> = take_vector();
    let row = dimension[0];
    let col = dimension[1];

    for i in 0..row{
        arr[i] = take_vector();
    }
    
    for i in 0..row{
        for j in 0..col{
            rotate[j][row-i-1] = arr[i][j];
        }
    }

    for i in 0..col{
        for j in 0..row-1{
            print!("{} ", rotate[i][j]);
        }
        println!("{}", rotate[i][row-1]);
    }
}