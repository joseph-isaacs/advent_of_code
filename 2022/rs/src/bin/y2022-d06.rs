use std::fs::File;
use std::io::{Read};


fn main() {
    let file = File::open("../input/y2022/d6.aoc.txt").unwrap();
    let mut string = String::new();
    std::io::BufReader::new(file).read_to_string(&mut string).unwrap();

    let idx = first_unique_seq_pos(string.as_str(), 4);
    println!("window 4 idx: {}", idx);

    let idx = first_unique_seq_pos(string.as_str(), 14);
    println!("window 14 idx: {}", idx);
}

fn first_unique_seq_pos(s: &str, w: usize) -> u64 {
    for (idx, window) in s.chars().collect::<Vec<char>>().windows(w).enumerate() {
        let mut window = window.to_vec();
        window.sort();
        window.dedup();
        if window.len() == w {
            return idx as u64 + w as u64;
        }
    }

    return 0
}

#[cfg(test)]
mod tests {

    #[test]
    fn ex0() {
        assert_eq!(super::first_unique_seq_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    }
    #[test]
    fn ex1() {
        assert_eq!(super::first_unique_seq_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(super::first_unique_seq_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    }

    #[test]
    fn ex2() {
        assert_eq!(super::first_unique_seq_pos("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    }

    #[test]
    fn ex3() {
        assert_eq!(super::first_unique_seq_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    }

    #[test]
    fn ex4() {
        assert_eq!(super::first_unique_seq_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }
}