use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use strum_macros::EnumString;


#[derive(Debug, PartialEq, EnumString)]
enum CommandDirection {
    #[strum(serialize = "U")]
    Up,
    #[strum(serialize = "D")]
    Down,
    #[strum(serialize = "L")]
    Left,
    #[strum(serialize = "R")]
    Right,
}

#[derive(Debug, PartialEq, EnumString)]
enum PositionMove {
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
    Right,
    UpRight,
}

#[derive(Debug, Hash, PartialOrd, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct RopeJoinPosition {
    head: Position,
    tail: Position
}

type RopePosition<const N: usize> = [Position; N];

#[derive(Debug)]
struct Command {
    command: CommandDirection,
    steps: u64,
}

fn head_is_touching_rope_tail(head: &Position, tail: &Position) -> bool {
    let xs =  (head.x - tail.x).abs() <= 1;
    let ys =  (head.y - tail.y).abs() <= 1;
    return xs && ys;
}

fn next_constrained_step(head: Position, tail: Position) -> Position {
    return match (head.x - tail.x, head.y - tail.y) {
        (2, 2) | (1, 2) | (2, 1) => Position { x: tail.x + 1, y: tail.y + 1},
        (-2, 2) | (-1, 2) | (-2, 1) => Position { x: tail.x - 1, y: tail.y + 1},
        (2, -2) | (1, -2) | (2, -1) => Position { x: tail.x + 1, y: tail.y - 1},
        (-2, -2) | (-1, -2) | (-2, -1) => Position { x: tail.x - 1, y: tail.y - 1},
        (2, 0) => Position { x: tail.x + 1, y: tail.y},
        (-2, 0) => Position { x: tail.x - 1, y: tail.y},
        (0, 2) => Position { x: tail.x, y: tail.y + 1},
        (0, -2) => Position { x: tail.x, y: tail.y - 1},
        (1, 0) | (1, 1) | (0, 1) | (-1, 1) | (-1, 0) | (-1, -1) | (0, -1) | (1, -1) | (0, 0) => tail,
        _ => panic!("diff {:?}", (head.x - tail.x, head.y - tail.y)),
    }
}

impl Command {
    fn next_steps<const N: usize>(&self, p: RopePosition<N>) -> (RopePosition<N>, HashSet<Position>) {
        let mut pos = HashSet::new();
        let mut p = p.clone();
        for _ in 0..self.steps {
            p = self.next_rope_step(p);
            pos.insert(p[N-1]);
        }

        return (p, pos)
    }

    fn next_rope_step<const N: usize>(&self, p: RopePosition<N>) -> RopePosition<N> {
        let mut new_pos = [Position { x: 0, y: 0 }; N];
        new_pos[0] = self.next_free_step(p[0]);
        for i in 1..N {
            new_pos[i] = next_constrained_step(new_pos[i-1], p[i]);
        }

        return new_pos
    }

    fn next_free_step(&self, p: Position) -> Position {
        return match self.command {
            CommandDirection::Up => Position { x: p.x, y: p.y + 1 },
            CommandDirection::Down => Position { x: p.x, y: p.y - 1 },
            CommandDirection::Left => Position { x: p.x - 1, y: p.y },
            CommandDirection::Right => Position { x: p.x + 1, y: p.y },
        };
    }
}

type Commands = Vec<Command>;

fn main() {
    let file = File::open("../input/y2022/d9.aoc.txt").unwrap();
    let mut string = String::new();
    std::io::BufReader::new(file).read_to_string(&mut string).unwrap();

    let commands = string
       .lines()
        .map(|line| { parse_command_lines(line).unwrap() })
        .collect::<Commands>();

    let mut pos = [ Position { x: 0, y: 0 }; 10];

    let mut prev_pos = HashSet::new();

    for c in commands.as_slice() {
        let (next_pos, interm) = c.next_steps(pos);
        prev_pos.extend(interm.iter().map(|c| *c));
        println!("c {:?}", pos);
        pos = next_pos;
    }

    // println!("commands {:?}", commands);
    // println!("prev_pos {:?}", prev_pos);
    println!("prev_pos {:?}", prev_pos.len());
}

fn parse_command_lines(command_str: &str) -> Option<Command> {
    let mut split = command_str.split(" ");
    if split.clone().count() != 2 {
        return None;
    }
    let direction = split.next().unwrap().parse::<CommandDirection>().ok()?;
    let steps = split.next().unwrap().parse::<u64>().ok()?;
    return Some(Command{ command: direction, steps: steps });
}
