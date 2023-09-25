use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space0};
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use strum_macros::EnumString;

#[derive(Debug)]
struct Monkey {
    id: usize,
    starting: Vec<u64>,
    code: MoneyCode,
}

#[derive(Debug)]
struct MoneyCode {
    statement: Statement,
    test: MoneyTest,
}

#[derive(Debug)]
struct Statement {
    operation: Operation,
    operands: (Operand, Operand),
}

impl Statement {
    pub(crate) fn apply(&self, state: u64) -> u64 {
        let left = match self.operands.0 {
            Operand::Item => state,
            Operand::Constant(c) => c,
        };
        let right = match self.operands.1 {
            Operand::Item => state,
            Operand::Constant(c) => c,
        };
        return match self.operation {
            Operation::Add => left + right,
            Operation::Multiply => left * right
        }
    }
}

#[derive(Debug)]
struct MoneyTest {
    devisor: u64,
    b_true: usize,
    b_false: usize,
}

impl MoneyTest {
    pub(crate) fn apply(&self, p0: u64) -> usize {
        if p0 % self.devisor == 0 {
            return self.b_true;
        } else {
            return self.b_false;
        }
    }
}

#[derive(Debug, PartialEq, EnumString)]
enum Operation {
    #[strum(serialize = "+")]
    Add,
    #[strum(serialize = "*")]
    Multiply,
}

#[derive(Debug)]
enum Operand {
    Item,
    Constant(u64)
}

 fn main() {
     let file = File::open("../input/y2022/d11.aoc.txt").unwrap();
     let mut string = String::new();
     std::io::BufReader::new(file).read_to_string(&mut string).unwrap();

     let (out, monkeys) = parse_monkeys(string.as_str()).unwrap();
     if out != "" {
         panic!("out is not empty {}", out);
     }
     println!("monkeys are {:?}", monkeys);

     let divisor_prod: u64 = monkeys.iter().map(|m| m.code.test.devisor).product();

     let mut states = monkeys.iter().map(|m| m.starting.clone()).collect::<Vec<_>>();

     let mut item_time = monkeys.iter().map(|m| 0).collect::<Vec<_>>();

     for _ in 0..10000 {
         for m in &monkeys {
             let current_state = states[m.id].clone();
             states[m.id] = Vec::new();

             for s in current_state.iter() {
                 item_time[m.id] += 1;
                 let new_s = m.code.statement.apply(*s) % divisor_prod;
                 let new_monkey = m.code.test.apply(new_s);
                 states[new_monkey].push(new_s);
             }
         }
     }

     println!("item_time are {:?}", item_time);
     item_time.sort_by(|a, b| a.cmp(b).reverse());
     println!("item_time are {:?}", item_time);
     println!("item_time are {:?}", (item_time[0] as u128) * (item_time[1] as u128));


}

fn starting_item(s: &str) -> IResult<&str, Vec<u64>> {
    let (out, items) = preceded(tag("Starting items: "), separated_list0(tag(", "), digit1))(s)?;
    return Ok((out, items.iter().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>()));
}

fn parse_operand(s: &str) -> Option<Operand> {
    if s == "old" {
        return Some(Operand::Item);
    } else {
        return Some(Operand::Constant(s.parse::<u64>().ok()?));
    }
}

fn parse_operation(s: &str) -> IResult<&str, Statement> {
    let operation = alt((tag("+"), tag("*")));
    let operand_space = preceded(space0, alt((tag("old"), digit1)));
    let operand_space2 = preceded(space0, alt((tag("old"), digit1)));
    let operation_space = preceded(space0, operation);
    let statement = preceded(tag("new ="), tuple((operand_space, operation_space, operand_space2)));
    let (out, (operand1, operation, operand2)) = preceded(tag("Operation: "), statement)(s)?;
    let statement = Statement{
        operation: Operation::from_str(operation).unwrap(),
        operands: (parse_operand(operand1).unwrap(), parse_operand(operand2).unwrap()),
    };
    return Ok((out, statement));
}

fn parse_m_test(s: &str) -> IResult<&str, MoneyTest> {
    let test_pred = terminated(preceded(tag("Test: divisible by "), digit1), newline);
    let t_branch = terminated(preceded(tuple((space0, tag("If true: throw to monkey "))), digit1), newline);
    let f_branch = preceded(tuple((space0, tag("If false: throw to monkey "))), digit1);

    let (out, (devisor, b_true, b_false)) = tuple((test_pred, t_branch, f_branch))(s)?;

    return Ok((out, MoneyTest {
        devisor: devisor.parse::<u64>().unwrap(),
        b_true: b_true.parse::<usize>().unwrap(),
        b_false: b_false.parse::<usize>().unwrap(),
    }));
}

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    let test_pred = terminated(delimited(tag("Monkey "), digit1, tag(":")), newline);
    let p_starting_item = delimited(space0, starting_item, newline);
    let p_parse_operation = delimited(space0, parse_operation, newline);
    let p_parse_m_test = delimited(space0, parse_m_test, newline);

    let (out, (m_id, starting, statement, test)) = tuple((test_pred, p_starting_item, p_parse_operation, p_parse_m_test))(s)?;

    return Ok((out, Monkey { id: m_id.parse::<usize>().unwrap(), starting, code: MoneyCode{ statement, test }} ))


}

fn parse_monkeys(s: &str) -> IResult<&str, Vec<Monkey>> {
    let (out, monkeys) = separated_list0(newline, parse_monkey)(s)?;

    return Ok((out, monkeys))
}