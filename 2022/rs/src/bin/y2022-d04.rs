use std::fs::File;
use std::io::BufRead;

#[derive(Debug,Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn parse_range(s: &str) -> Range {
        let mut parts = s.split("-");
        let start = parts.next().unwrap().parse::<u64>().unwrap();
        let end = parts.next().unwrap().parse::<u64>().unwrap();
        return Range { start, end };
    }

    fn contains(&self, other: &Range) -> bool {
        return self.start <= other.start && other.end <= self.end;
    }

    fn overlap(&self, other: &Range) -> bool {
        return self.start <= other.end && self.end >= other.start
    }
}


fn main() {
    let file = File::open("../input/y2022/d4.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines()
        .filter_map(|line| line.ok()).collect::<Vec<_>>();

    let both = lines.iter().map(|line| {
        let mut parts = line.split(",");
        let one = &Range::parse_range(parts.next().unwrap());
        let two = &Range::parse_range(parts.next().unwrap());

        return ((one.contains(two) || two.contains(one)) as u64, one.overlap(two) as u64);
    }).fold((0, 0), |acc, (contained, overlap)| {
        return (acc.0 + contained, acc.1 + overlap);
    });

    println!("contained_lines: {}", both.0);
    println!("overlap_lines: {}", both.1);
}