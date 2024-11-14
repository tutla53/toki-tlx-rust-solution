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

fn print_ancestor(parent: &HashMap<String, String>, keyword: &String, target: &String ) -> bool{
    let mut st: Vec<String> = Vec::new();
    let mut found: bool = false;
    let mut key_species = keyword.to_string();

    st.push(keyword.to_string());
    
    while !found {
        match parent.get_key_value(&key_species){
            Some((_key, value)) => {
                st.push(value.to_string());
                if value == target {found = true;}
                key_species = value.to_string();
            },
            None => {
                break;
            },
        }
    }

    if found {
        for value in st.into_iter().rev(){
            println!("{}", value);
        }
    }

    return found;
}

fn main(){
    let mut parent: HashMap<String, String> = HashMap::new();
    let cmd: Vec<u16> = take_vector();

    for _i in 0..cmd[1] {
        let species: Vec<String> = take_vector();
        parent.insert(species[1].to_string(),species[0].to_string());
    }

    let guess: Vec<String> = take_vector();  
    let target1 = guess[0].to_string();
    let target2 = guess[1].to_string();
    
    let f1 = print_ancestor(&parent, &target1, &target2);
    let f2 = print_ancestor(&parent, &target2, &target1);

    if (f1 == false) && (f2 == false) {println!("TIDAK MUNGKIN");}

}