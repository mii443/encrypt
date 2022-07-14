
use std::collections::VecDeque;

fn main() {
    println!("testing binary");

    let mut queue: VecDeque<i64> = VecDeque::new();

    queue.push_back(1i64);
    queue.push_back(2i64);
    queue.push_back(3i64);

    for x in 0..queue.len() {
        println!("{}", queue[x]);
    }

    println!("{:?}", queue);
    println!("{:?}", queue.len());
}