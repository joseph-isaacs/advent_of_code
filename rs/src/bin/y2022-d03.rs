use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::iter::Peekable;

struct LineTripleIterator<I>
    where
        I: Iterator<Item = String>,
{
    inner: Peekable<I>,
}

impl<I> LineTripleIterator<I>
    where
        I: Iterator<Item = String>,
{
    fn new(iter: I) -> Self {
        LineTripleIterator { inner: iter.peekable() }
    }
}


impl<I> Iterator for LineTripleIterator<I>
    where
        I: Iterator<Item = String>,
{
    type Item = (String, String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.peek().is_none() {
            return None;
        }
        return Some((self.inner.next().unwrap(), self.inner.next().unwrap(), self.inner.next().unwrap()))
    }
}

fn main() {
    let file = File::open("../input/y2022/d3.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines()
        .filter_map(|line| line.ok()).collect::<Vec<_>>();

    let cost: u64 = lines.iter().map(|line| {
        let (first, second) = split_bag(&line);

        return first
            .intersection(&second)
            .next()
            .map(|c| score_char(*c))
            .unwrap_or(0)
    }).sum();

    let three_elf_cost: u64 = LineTripleIterator::new(lines.iter().map(|c| c.clone())).map(|(l1, l2, l3)| {

        let line1 = l1.chars().collect::<HashSet<_>>();
        let line2 = l2.chars().collect::<HashSet<_>>();
        let line3 = l3.chars().collect::<HashSet<_>>();

        return line1
            .intersection(&line2)
            .map(|c| *c)
            .collect::<HashSet<_>>()
            .intersection(&line3)
            .next()
            .map(|c| score_char(*c))
            .unwrap_or(0)
    }).sum();

    println!("cost: {}", cost);
    println!("three elf cost: {}", three_elf_cost);
}

fn split_bag(line: &String) -> (HashSet<char>, HashSet<char>) {
    let first = line[..line.len() / 2].chars().collect::<HashSet<_>>();
    let second = line[line.len() / 2..].chars().collect::<HashSet<_>>();
    (first, second)
}

fn score_char(p0: char) -> u64 {
    match p0 as u64 {
        97..=122 => p0 as u64 - 96,
        65..=90 => p0 as u64 - 38,
        _ => panic!("Unexpected char: {}", p0)
    }
}