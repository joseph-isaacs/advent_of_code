#![feature(iter_intersperse)]
extern crate core;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{Read};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while};
use nom::character::complete::{alpha1, char, digit1, newline, space1};
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{preceded, terminated, tuple};

#[derive(Debug)]
enum Command {
    CD(String),
    LS(Vec<FlatFolder>),
}

#[derive(Debug, Clone)]
enum FlatFolder {
    Directory(String),
    File(String, u64),
}

fn until_newline(s: &str) -> IResult<&str, &str> {
    return Ok(take_till(|c| c == ' ' || c == '\n')(s)?);
}

fn parse_file(s: &str) -> IResult<&str, FlatFolder> {
    let (s, (size, _, name)) = tuple((digit1, space1, until_newline))(s)?;
    return Ok((s, FlatFolder::File(String::from(name), size.parse::<u64>().unwrap())));
}

fn parse_folder(s: &str) -> IResult<&str, FlatFolder> {
    let (s, (_, _, name)) = tuple((tag("dir"), space1, alpha1))(s)?;
    return Ok((s, FlatFolder::Directory(String::from(name))));
}

fn parse_ls(s: &str) -> IResult<&str, Command> {
    let (s, _) = terminated(tag("ls"), newline)(s)?;
    let (s, folders) = many1(terminated(alt((parse_folder, parse_file)), newline))(s)?;
    return Ok((s, Command::LS(folders)));
}
fn parse_cd(s: &str) -> IResult<&str, Command> {
    let (s, (_, _, dir, _)) = tuple((tag("cd"), space1, until_newline, newline))(s)?;
    return Ok((s, Command::CD(String::from(dir))));
}



fn parse_command_lines(s: &str) -> IResult<&str, Vec<Command>> {
    let parse_command = preceded(tuple((char('$'), space1)), alt((parse_ls, parse_cd)));
    let (s, commands) = many1(parse_command)(s)?;

    return Ok((s, commands));
}

fn create_dir_structure(commands: &[Command]) -> HashMap<Vec<&str>, &[FlatFolder]> {
    let mut directory: HashMap<Vec<&str>, &[FlatFolder]> = HashMap::new();
    let mut directory_size: HashMap<Vec<&str>, u64> = HashMap::new();


    let mut dir_stack =  Vec::new();
    for c in commands {
        match c {
            Command::CD(dir) => {
                match dir.as_str() {
                    ".." => {
                        dir_stack.pop();
                        if let Some(folders) = directory.get(&dir_stack) {
                            if let Some(size) = try_find_size(&folders.to_vec(), &mut directory_size, &dir_stack) {
                                directory_size.insert(dir_stack.clone(), size);
                            }
                        }
                    },
                    _ => { dir_stack.push(dir.as_str()); },
                };
            }
            Command::LS(folders) => {
                if let Some(x) = directory.insert(dir_stack.clone(), folders.as_slice()) {
                    panic!("duplicate directory: {:?}", x);
                }
                if let Some(size) = try_find_size(folders, &mut directory_size, &dir_stack) {
                    directory_size.insert(dir_stack.clone(), size);
                }

            }
        }
    }

    while let Some(_) = dir_stack.pop() {
        if let Some(folders) = directory.get(&dir_stack) {
            if let Some(size) = try_find_size(&folders.to_vec(), &mut directory_size, &dir_stack) {
                directory_size.insert(dir_stack.clone(), size);
            }
        }
    }

    let s: u64 = directory_size.values().filter(|x| **x < 100_000).map(|x| *x).sum();
    println!("sum {}", s);

    let cap = 70_000_000u64;
    let used: u64 = commands.iter().filter_map(|c| match c {
        Command::LS(folders) => Some(folders),
        _ => None,
    })
    .map(|f| {
        f.iter().map(|f| match f {
            FlatFolder::Directory(_) => 0,
            FlatFolder::File(_, size) => *size,
        }).sum::<u64>()
    })
    .sum();

    let full = cap - used;

    let min  = directory_size.values().filter(|x| **x > (30_000_000 - full)).map(|x| *x).min();
    println!("min {:?}", min);

    return directory
}

fn try_find_size(folders: &Vec<FlatFolder>, directory_size: &mut HashMap<Vec<&str>, u64>, dir_stack: &Vec<&str>) -> Option<u64> {
    return folders.iter().map(|f| match f {
        FlatFolder::Directory(d) => {
            let mut new_dir = dir_stack.clone();
            new_dir.push(d.as_str());
            directory_size.get(&new_dir)
        },
        FlatFolder::File(_, size) => Some(size),
    }).fold(Some(0), |acc, x| {
        match (acc, x) {
            (Some(acc), Some(x)) => Some(acc + x),
            _ => None,
        }
    })
}

fn main() {
    let file = File::open("../input/y2022/d7.aoc.txt").unwrap();
    let mut string = String::new();
    std::io::BufReader::new(file).read_to_string(&mut string).unwrap();

    let (out, commands) = parse_command_lines(string.as_str()).unwrap();

    let cds = commands.iter().filter_map(|c| {
        match c {
            Command::CD(dir) => match dir.as_str() {
                ".." => None,
                _ => Some(dir),
            }
            _ => None,
        }
    }).collect::<Vec<_>>();

    let unique_cds = cds.iter().map(|c| *c).collect::<HashSet<_>>();

    let non_uniq_cds = cds.iter().filter(|c| !unique_cds.contains(*c)).collect::<Vec<_>>();

    let structure = create_dir_structure(commands.as_slice());

    println!("hello")


}

// fn main() {
//     let mut hashmap: HashMap<Vec<u32>, String> = HashMap::new();
//
//     let key1 = vec![1, 2, 3];
//     let key11 = key1.clone();
//     let key2 = vec![4, 5, 6];
//     let key22 = vec![4, 5, 6];
//
//     hashmap.insert(key1.clone(), "Value 1".to_string());
//     hashmap.insert(key2.clone(), "Value 2".to_string());
//
//     // Retrieve values using Vec keys
//     if let Some(value) = hashmap.get(&key11) {
//         println!("Value for key1: {}", value);
//     }
//
//     if let Some(value) = hashmap.get(&key22) {
//         println!("Value for key2: {}", value);
//     }
// }