use {
    std::io::stdin,
    std::str::FromStr,
    std::fmt::Debug,
    std::collections::HashMap
};

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
    let mut map: HashMap<String, String> = HashMap::new();
    let cmd: Vec<u32> = take_vector();
    let mut n = cmd[0];
    let mut q = cmd[1];

    while n>0 {
        let data: Vec<String> = take_vector();
        map.insert(data[0].clone(), data[1].clone());
        
        n = n-1;
    }
    
    while q>0 {
        let input = to_string(take_string());

        match map.get_key_value(&input){
            Some((_key, value)) => println!("{}", *value),
            None => println!("NIHIL"),
        }

        q = q-1;
    }
}