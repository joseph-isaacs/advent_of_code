use std::fs::File;
use std::io::{Read};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1, newline};
use nom::character::complete::char as nchar;
use nom::combinator::opt;
use nom::IResult;
use nom::multi::{many1, many_till};
use nom::sequence::{delimited, terminated, tuple};

struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

struct StartStack {
    a: Vec<char>,
}

fn parse_crate(s: &str) -> IResult<&str, char> {
    let (out, c) = delimited(nchar('['), anychar, nchar(']'))(s)?;
    return Ok((out, c));
}

fn parse_crate_opt(s: &str) -> IResult<&str, Option<char>> {
    let parse_crate_some = |s| parse_crate(s).map(|(s, c)| (s, Some(c)));
    let parse_crate_missing = |s| tag("   ")(s).map(|(s, c)| (s, None));
    return alt((parse_crate_some, parse_crate_missing))(s);
}

fn parse_line(s: &str) -> IResult<&str, Vec<Option<char>>> {
    let spaces = alt((tag(" "), tag("")));
    let (out, (c, _)) = many_till(terminated(parse_crate_opt, spaces), newline)(s)?;

    return Ok((out, c));
}

fn number_lines(s: &str) -> IResult<&str, ()> {
    let num = tuple((tag(" "), anychar, tag(" ")));
    let spaces = alt((tag(" "), tag("")));
    let (out, _) = many_till(terminated(num, spaces), newline)(s)?;
    return Ok((out, ()));
}

fn parse_lines(s: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (out, (c, _)) = many_till(parse_line, number_lines)(s)?;
    let (out, _) = newline(out)?;


    let stack_count = c[0].len();
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    for s in c.iter().rev() {
        for (i, c) in s.iter().enumerate() {
            if let Some(c) = c {
                stacks[i].push(*c);
            }
        }
    }


    return Ok((out, stacks));
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    let (out, (_, count, _, from, _, to)) = tuple((tag("move "), digit1, tag(" from "), digit1, tag(" to "), digit1))(s)?;

    return Ok((out, Instruction {
        from: from.parse::<usize>().unwrap(),
        to: to.parse::<usize>().unwrap(),
        count: count.parse::<usize>().unwrap(),
    }));
}

fn parse_instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
    let (out, instructions) = many1(terminated(parse_instruction, opt(newline)))(s)?;
    return Ok((out, instructions));
}

fn parse_input(s: &str) -> Option<(Vec<Vec<char>>, Vec<Instruction>)> {
    let (out, start_lines) = parse_lines(s).ok()?;
    let (_, instruction) = parse_instructions(out).ok()?;

    return Some((start_lines, instruction));
}


fn main() {
    let file = File::open("../input/y2022/d5.aoc.txt").unwrap();
    let mut string = String::new();
    std::io::BufReader::new(file).read_to_string(&mut string).unwrap();

    let (state, instructions) = parse_input(&string).unwrap();

    let state_9000_top = apply_instructions_9000(state.clone(), &instructions)
        .iter()
        .map(|s| s.last().unwrap())
        .collect::<String>();

    println!("9000 {:?}", state_9000_top);

    let state_9001_top = apply_instructions_9001(state.clone(), &instructions)
        .iter()
        .map(|s| s.last().unwrap())
        .collect::<String>();

    println!("9001 {:?}", state_9001_top);
}

fn apply_instruction_9000(mut state: Vec<Vec<char>>, instruction: &Instruction) -> Vec<Vec<char>>{
    for _ in 0..instruction.count {
        let c = state[instruction.from - 1].pop().unwrap();
        state[instruction.to - 1].push(c);
    }
    return state
}

fn apply_instructions_9000(mut state: Vec<Vec<char>>, instructions: &Vec<Instruction>) -> Vec<Vec<char>> {
    for i in instructions {
        state = apply_instruction_9000(state, i);
    }
    return state
}

fn apply_instruction_9001(mut state: Vec<Vec<char>>, instruction: &Instruction) -> Vec<Vec<char>> {
    let mut v = Vec::new();
    for _ in 0..instruction.count {
        let c = state[instruction.from - 1].pop().unwrap();
        v.push(c)
    }
    state[instruction.to - 1].extend(v.iter().rev());

    return state;
}

fn apply_instructions_9001(mut state: Vec<Vec<char>>, instructions: &Vec<Instruction>) -> Vec<Vec<char>> {
    for i in instructions {
        state = apply_instruction_9001(state, i);
    }
    return state
}