use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("../input/y2022/d1-test.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(0u64));
    heap.push(Reverse(0u64));
    heap.push(Reverse(0u64));
    let mut curr = 0;
    reader.lines().filter_map(|line| line.ok()).for_each(|line| {
        if line == "" {
            // max of two values
            heap.push(Reverse(curr));
            heap.pop();
            curr = 0;
        } else {
            curr += line.parse::<u64>().unwrap();
        }
    });

    let third = heap.pop().unwrap().0;
    let second = heap.pop().unwrap().0;
    let first = heap.pop().unwrap().0;

    println!("first: {}", first);
    println!("second: {}", second);
    println!("third: {}", third);
    println!("sum: {}", first + second + third);
}