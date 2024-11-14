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

fn main(){
    let mut q: u16 = take_int();
    let mut arr: Vec<Vec<u16>> = vec![vec![0; 1]; 1010];

    while q>0 {
        let cmd :Vec<String> = take_vector();
        match cmd[0].as_str() {
            "add" => {
                let l: usize = cmd[1].parse().unwrap();
                let x: u16 = cmd[2].parse().unwrap();
                let y: u16 = cmd[3].parse().unwrap();

                for _i in 0..y { arr[l].push(x); }

            },
            "out" => {
                let l: usize = cmd[1].parse().unwrap();
                let z: usize = cmd[2].parse().unwrap();
                println!("{}", arr[l][z]);
            },
            _ =>{

            },
        }
        q = q - 1;
    }
}