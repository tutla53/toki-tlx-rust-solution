use {
    std::io::stdin,
    std::str::FromStr,
    std::fmt::Debug,
    std::collections::VecDeque,
    std::cmp::max
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
                let mut y: u16 = cmd[1].parse().unwrap();
                count = max(0, count - y as u32);

                if face == false {
                    match q.get(0){
                        Some(value) => {
                            println!("{}", value.data);
                        },
                        None => println!(""), 
                    }
                    
                    while y>0 {
                        match q.get_mut(0){
                            Some(value) => {
                                if value.freq > y {
                                    value.freq =  value.freq - y;
                                    break;
                                }
                                else{
                                    y = y - value.freq;
                                    q.pop_front(); 
                                }
                            },
                            None => break,
                        }
                    }
                }

                if face == true {
                    match q.get(q.len()-1){
                        Some(value) => {
                            println!("{}", value.data);
                        },
                        None => println!(""), 
                    }
                    
                    while y>0 {
                        match q.get_mut(q.len()-1){
                            Some(value) => {
                                if value.freq > y {
                                    value.freq =  value.freq - y;
                                    break;
                                }
                                else{
                                    y = y - value.freq;
                                    q.pop_back(); 
                                }
                            },
                            None => break,
                        }
                    }
                }
            },

            _=>{
                face = !face;
            },
        }
        
        n = n-1;
    }
}