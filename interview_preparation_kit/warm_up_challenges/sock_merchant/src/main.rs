// https://www.hackerrank.com/challenges/sock-merchant/problem?h_l=interview&playlist_slugs%5B%5D=interview-preparation-kit&playlist_slugs%5B%5D=warmup
use std::collections::HashMap;
use std::io;

fn sock_merchant(array: Vec<&str>) -> i32 {
    let mut socks_num = 0;
    let mut counter: HashMap<&str, i32> = HashMap::new();

    for item in array.iter() {
        let var = counter.entry(item).or_insert(0);
        *var += 1;
    }


    for item in counter.values() {
        socks_num += (item - (item%2)) / 2
    }
    socks_num
}

fn main() {
    let mut _n = String::new();
    let mut arr = String::new();

    io::stdin().read_line(&mut _n)
        .expect("Failed to read from stdin");

    io::stdin().read_line(&mut arr)
        .expect("Failed to read from stdin");

    let array: Vec<&str> = arr.trim().split(' ').collect();

    println!("{:?}", sock_merchant(array));
}
