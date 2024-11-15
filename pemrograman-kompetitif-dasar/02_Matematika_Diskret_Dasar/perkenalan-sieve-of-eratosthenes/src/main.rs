use {
    std::io::stdin,
    std::str::FromStr,
    std::fmt::Debug
};

const N: usize = 1000000;
const K: usize = 80000;

fn take_int<T: FromStr>() -> T where <T as FromStr>::Err: Debug{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap()
}

fn sieve(sieve_array: &mut Vec<bool>, n: usize){
    sieve_array[0] = false;
    sieve_array[1] = false;

    for i in (4..n+1).step_by(2) { sieve_array[i] = false; }

    let mut i = 3;
    while i*i<= n+1 {
        if sieve_array[i] {
            for j in (i*i..n+1).step_by(2*i){ 
                sieve_array[j] = false; 
            }
        }
        i+=2;
    }
}

fn main() {
    let mut is_prime: Vec<bool> = vec![true; N+1];
    let mut prime: Vec<u32> = vec![0;K+10];
    let mut idx: usize = 0;

    sieve(&mut is_prime, N);

    let mut test_case:u32 = take_int();

    for i in 0..N {
		if is_prime[i]==true {
			prime[idx]= i as u32;
			idx +=1;
		}
	}    

    while test_case>0 {
    	let x:usize = take_int();
    	println!("{}",prime[x-1]);
        test_case = test_case - 1;
    }
}