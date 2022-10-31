use std::time::{Instant, Duration};

use radix_rs::{sort, utils};

fn main() {
    let mut arr: Vec<u32> = vec![0; 1000000];
    utils::fill_array(&mut arr);

    let start: Instant = Instant::now();
    sort::radix(&mut arr);
    let duration: Duration = start.elapsed();

    println!("Le temps du tri radix de {} elements en rust est: {:?}", arr.len(), duration);
}
