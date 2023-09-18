use std::fs::File;
use std::io::BufRead;

#[derive(Debug,Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn score(&self) -> u64 {
        return match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }

    fn outcome(c: char) -> Option<Outcome> {
        return match c {
            'X' => Some(Outcome::Lose),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Win),
            _ => None
        }
    }

    fn find_our_move_given_theirs(&self, their_move: Move) -> Move {
        return match self {
            Outcome::Win => match their_move {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            Outcome::Draw => match their_move {
                Move::Rock => Move::Rock,
                Move::Paper => Move::Paper,
                Move::Scissors => Move::Scissors,
            },
            Outcome::Lose => match their_move {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
        }
    }
}

#[derive(Debug,Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}



impl Move {
    fn their_move(c: char) -> Option<Move> {
        return match c {
            'A' => Some(Move::Rock),
            'B' => Some(Move::Paper),
            'C' => Some(Move::Scissors),
            _ => None
        }
    }

    fn our_move(c: char) -> Option<Move> {
        return match c {
            'X' => Some(Move::Rock),
            'Y' => Some(Move::Paper),
            'Z' => Some(Move::Scissors),
            _ => None
        }
    }

    fn score_choice(&self) -> u64 {
        return match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }

    fn game_outcome(&self, other: Move) -> Outcome {
        return match self {
            Move::Rock => match other {
                Move::Rock => Outcome::Draw,
                Move::Paper => Outcome::Lose,
                Move::Scissors => Outcome::Win
            },
            Move::Paper => match other {
                Move::Rock => Outcome::Win,
                Move::Paper => Outcome::Draw,
                Move::Scissors => Outcome::Lose
            },
            Move::Scissors => match other {
                Move::Rock => Outcome::Lose,
                Move::Paper => Outcome::Win,
                Move::Scissors => Outcome::Draw
            }
        }
    }
}

fn main() {
    let file = File::open("../input/y2022/d2.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut p1_game_scores = 0;
    let mut p2_game_scores = 0;
    reader.lines().filter_map(|line| line.ok()).for_each(|line| {
        let moves = line.split(" ").collect::<Vec<_>>();
        let their_move = Move::their_move(moves[0].chars().next().unwrap()).unwrap();

        // p1
        {
            let our_move = Move::our_move(moves[1].chars().next().unwrap()).unwrap();
            let p1_game_score = our_move.game_outcome(their_move.clone()).score();
            p1_game_scores += p1_game_score + our_move.score_choice();
        }

        // p2
        {
            let outcome = Outcome::outcome(moves[1].chars().next().unwrap()).unwrap();
            let our_move = outcome.find_our_move_given_theirs(their_move.clone());
            p2_game_scores += outcome.score() + our_move.score_choice();
        }
    });

    println!("P1 game_scores: {}", p1_game_scores);
    println!("P2 game_scores: {}", p2_game_scores);
}