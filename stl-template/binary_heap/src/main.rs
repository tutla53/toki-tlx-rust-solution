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

fn main(){
    let mut heap = BinaryHeap::new();
    let cmd: Vec<usize> = take_vector();
    let arr: Vec<u32> = take_vector();

    for i in 0..cmd[0]{
        if heap.len()<cmd[1] {
            heap.push(arr[i]);
        }
        else{
            match heap.peek(){
                None => println!(""),
                Some(value) => {
                    if arr[i]<*value {
                        heap.pop();
                        heap.push(arr[i]);
                    }
                },
            }
        }
    }
    
    match heap.peek(){
        None => println!(""),
        Some(value) => {
            println!("{}", value);
        },
    }
}