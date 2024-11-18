use {
    std::io::stdin,
    std::str::FromStr,
    std::fmt::Debug
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
    let mut stay_cost: Vec<u8> = vec![0];
    let mut virus_list: Vec<String> = Vec::with_capacity(15);
    let mut parent: Vec<u8>

    let cmd: Vec<u32> = take_vector();
    let n = cmd[0];
    let s = cmd[1] as u8;
    let p = cmd[2] as u8;

    for _i in 0..n {
        let new: u8 = take_int();
        stay_cost.push(new);
    }

    for _i in 0..p {
        let new: Vec<String> = take_vector();
        
    }


}