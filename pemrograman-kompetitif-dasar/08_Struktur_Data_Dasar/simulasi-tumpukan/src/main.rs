use {
    std::io::stdin,
    std::str::FromStr,
    std::fmt::Debug,
    std::cmp::max
};

struct StackData{
    data: i32,
    freq: i32
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
    let mut st: Vec<StackData> = Vec::new();
    let mut q: u16 = take_int();
    let mut count: u32 = 0;

    while q>0 {
        let cmd: Vec<String> = take_vector();
        
        match cmd[0].as_str() {
            "add" => {
                let x: i32 = cmd[1].parse().unwrap();
                let y: i32 = cmd[2].parse().unwrap();

                count = count + y as u32;
                st.push(StackData{data: x, freq: y});
                println!("{}", count);
            },

            "del" => {
                let mut y: i32 = cmd[1].parse().unwrap();
                count = max(0, count - y as u32);
                
                match st.last() {
                    Some(value) => {
                        println!("{}", value.data);
                    },
                    None => println!(""), 
                }
                
                while y>0 {
                    match st.last_mut() {
                        Some(value) => {
                            if value.freq > y {
                                value.freq =  value.freq - y;
                                break;
                            }
                            else{
                                y = y - value.freq;
                                st.pop(); 
                            }
                        },
                        None => break,
                    }
                }
            },

            "adx" => {
                let x: i32 = cmd[1].parse().unwrap();

                for item in &mut st{
                    item.data = item.data + x;
                }

            },

            _ =>{
                let x: i32 = cmd[1].parse().unwrap();

                for item in &mut st{
                    item.data = item.data - x;
                }
            },
        }

        q = q-1;
    }
}