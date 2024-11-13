use {
    std::io::stdin,
    std::str::FromStr,
    std::fmt::Debug,
    std::collections::BinaryHeap
};

fn take_vector<T: FromStr>() -> Vec<T> where <T as FromStr>::Err: Debug{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let arr: Vec<T> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    return arr;
}

fn take_int<T: FromStr>() -> T where <T as FromStr>::Err: Debug{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap()
}

fn main(){
    let mut heap = BinaryHeap::new();
    let mut q: u32 = take_int();
    
    while q>0 {
        let cmd: Vec<u32> = take_vector();
        match cmd[0] {
            1 => {
                heap.push(cmd[1]);
            },
            2 => {
                match heap.peek(){
                    None => println!(""),
                    Some(value) => println!("{}", value),
                }
            },
            _ => {
                heap.pop();
            },
        }
        q = q-1;
    }
}