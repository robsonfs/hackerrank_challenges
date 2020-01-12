// https://www.hackerrank.com/challenges/jumping-on-the-clouds/problem?h_l=interview&playlist_slugs%5B%5D%5B%5D=interview-preparation-kit&playlist_slugs%5B%5D%5B%5D=warmup&isFullScreen=true
use std::io;

fn jumping_on_clouds(c: Vec<&str>) -> usize {
    let mut jumps = 0;
    let last_position = c.len() - 1;
    let mut make_jump = |current_position: usize| {
        let mut new_position = current_position + 2;
        if (new_position > last_position) || (c[new_position] == "1") {
            new_position = current_position + 1;
        };
        jumps += 1;
        new_position
    };
    let mut position = 0;
    loop {
        position = make_jump(position);
        if position == last_position {
            break;
        }
    }
    jumps
}

fn main() {
    let mut _n = String::new();
    let mut clouds = String::new();

    io::stdin().read_line(&mut _n)
        .expect("Failed to read first input");
    io::stdin().read_line(&mut clouds)
        .expect("Failed to read second input");

    let c: Vec<&str> = clouds.trim().split(' ').collect();
    println!("{}", jumping_on_clouds(c));
}
