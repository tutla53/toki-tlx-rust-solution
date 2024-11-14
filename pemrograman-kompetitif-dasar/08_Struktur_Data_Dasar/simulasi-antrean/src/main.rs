use {
    std::io::stdin,
    std::str::FromStr,
    std::fmt::Debug,
    std::collections::VecDeque
};

struct QueueData{
    data: u16,
    freq: u16
}

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

fn main(){
    let mut q: VecDeque<QueueData> = VecDeque::new();
    let mut face: bool = false;
    let mut count: u32 = 0;
    let mut n: u16 = take_int();

    
    while n>0 {
        let cmd: Vec<String> = take_vector();
        match cmd[0].as_str() {
            "add" => {
                let x = cmd[1].parse().unwrap();
                let y = cmd[2].parse().unwrap();
                let new = QueueData{data: x, freq: y};

                if face == false {
                    q.push_back(new);
                }
                else{
                    q.push_front(new);
                }
                count = count + y as u32;
                println!("{}", count);
            },
            "del" => {

            },
            _=>{
                face = !face;
            },
        }
        n = n-1;
    }

}