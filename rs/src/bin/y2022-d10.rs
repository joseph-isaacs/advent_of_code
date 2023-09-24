
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq)]
enum Instruction {
    Add(i64),
    Noop
}

impl FromStr for Instruction {
    type Err = String;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(instr_str: &str) -> Result<Self, Self::Err> {
        if instr_str == "noop" {
            return Ok(Instruction::Noop);
        }

        let mut split = instr_str.split(" ");
        if split.clone().count() != 2 {
            return Err("wrong format".to_string());
        }
        if split.next().unwrap() == "addx" {
            let value = split.next().unwrap().parse::<i64>().map_err(|e| e.to_string())?;
            return Ok(Instruction::Add(value));
        }

        return Err("not noop or addx".to_string());

    }
}


// fn main() {
//     let file = File::open("../input/y2022/d10.aoc.txt").unwrap();
//     let mut string = String::new();
//     std::io::BufReader::new(file).read_to_string(&mut string).unwrap();
//
//     let mut reg_val = 1;
//     // let mut cycle = 0;
//     let mut states = vec![reg_val];
//     for line in string.lines() {
//         let instruction = line.parse::<Instruction>().unwrap();
//         println!("instruction is {:?}", instruction);
//         match instruction {
//             Instruction::Noop => {states.push(reg_val);},
//             Instruction::Add(value) => {states.push(reg_val); reg_val += value; states.push(reg_val)},
//         }
//     }
//
//     println!("state as cycle 20 is {}", states[20-1]);
//     println!("state as cycle 60 is {}", states[60-1]);
//
//     let str_20 = states[20-1] * 20;
//     let str_60 = states[60-1] * 60;
//     let str_100 = states[100-1] * 100;
//     let str_140 = states[140-1] * 140;
//     let str_180 = states[180-1] * 180;
//     let str_220 = states[220-1] * 220;
//
//     let sum = str_20 + str_60 + str_100 + str_140 + str_180 + str_220;
//
//     println!("sum is {}", sum);
// }

 fn main() {
    let file = File::open("../input/y2022/d10.aoc.txt").unwrap();
    let mut string = String::new();
    std::io::BufReader::new(file).read_to_string(&mut string).unwrap();

    let mut reg_val = 1;
    let mut cycle = 0;
    let mut screen = String::new();
    for line in string.lines() {
        let instruction = line.parse::<Instruction>().unwrap();

        match instruction {
            Instruction::Noop => {
                screen += pixel_value(cycle, reg_val).as_str();
                cycle += 1;
            },
            Instruction::Add(value) => {
                screen += pixel_value(cycle, reg_val).as_str();
                cycle += 1;
                screen += pixel_value(cycle, reg_val).as_str();
                cycle += 1;
                reg_val += value;
            },
        }
    }

    // println!("state as cycle 20 is {}", states[20-1]);
    // println!("state as cycle 60 is {}", states[60-1]);
    //
    // let str_20 = states[20-1] * 20;
    // let str_60 = states[60-1] * 60;
    // let str_100 = states[100-1] * 100;
    // let str_140 = states[140-1] * 140;
    // let str_180 = states[180-1] * 180;
    // let str_220 = states[220-1] * 220;
    //
    // let sum = str_20 + str_60 + str_100 + str_140 + str_180 + str_220;
    //
    // println!("sum is {}", sum);
     println!("{}", screen);
}

fn pixel_value(cycle: i64, reg: i64) -> String {
    let mut ret = String::new();
    if cycle != 0 && cycle % 40 == 0 {
        ret += "\n";
    }
    let x_pos = cycle % 40;
    if (x_pos - reg).abs() <= 1 {
        ret += "#";
    } else {
        ret += ".";
    }
    return ret
}

