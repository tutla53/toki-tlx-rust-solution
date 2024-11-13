use {
    std::io::stdin
};

fn take_string() -> Vec<char> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let vec:Vec<char> = input.trim().chars().collect();
    return vec;
}

fn main(){
    let input_word = take_string();
    let word_len = input_word.len();
    let sq_len = (word_len as f32).sqrt().ceil() as usize;
    let mut idx: usize = 0;

    for i in 0..sq_len{
        for _j in 0..sq_len{
            if idx< word_len {print!("{}", input_word[idx]);}
            else { print!(".");}

            if i&1 == 0 {idx = idx + 1;}
            else{idx = idx - 1;}
        }
        
        if i&1 == 0 {idx = idx + (sq_len-1);}
        else {idx = idx + (sq_len+1);}
        println!("");
    }
}