#![feature(iter_intersperse)]
#![feature(slice_iter_mut_as_mut_slice)]
extern crate core;

use std::cmp::max;
use std::fs::File;
use std::io::Read;

#[derive(Debug,Clone)]
struct ColumnIterator<'a, T> {
    grid: &'a  [Box<[T]>],
    pos: usize,
    back_pos: usize,
    col: usize,
}

impl<'a, T> ColumnIterator<'a, T> {
    fn new(grid: &'a [Box<[T]>], col: usize) -> Self {
        ColumnIterator {
            grid,
            pos: 0,
            back_pos: if grid.len() == 0 { 0 } else { grid.len() },
            col,
        }
    }
}

impl <'a, T> Iterator for ColumnIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.grid.len() == 0 {
            return None;
        }
        let ll = self.grid.len();
        if self.pos >= ll {
            return None;
        }
        let res = Some(&self.grid[self.pos][self.col]);
        self.pos += 1;
        return res;
    }
}

impl<'a, T> DoubleEndedIterator for ColumnIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back_pos <= 0 {
            return None;
        }
        self.back_pos -= 1;
        let res = Some(&self.grid[self.back_pos][self.col]);
        return res;
    }
}

// #[derive(Debug)]
// struct MutColumnIterator<'a, T> {
//     grid: &'a mut Vec<Box<[T]>>,
//     pos: usize,
//     col: usize,
// }
//
// impl<'a, T> MutColumnIterator<'a, T> {
//     fn new(grid: &'a mut Vec<Box<[T]>>, col: usize) -> Self {
//         MutColumnIterator {
//             grid,
//             pos: 0,
//             col,
//         }
//     }
//
//     fn ret(&'a mut self) -> &'a mut Vec<Box<[T]>> {
//         return self.grid
//     }
// }
//
// impl <'a, T> Iterator for MutColumnIterator<'a, T> {
//     type Item = &'a mut T;
//
//     fn next(&'a mut self) -> Option<Self::Item> {
//         let ll = self.grid[self.col].len();
//         if self.pos >= ll {
//             return None;
//         }
//         let res = Some(&mut self.grid[self.col][self.pos]);
//         self.pos += 1;
//         return res;
//     }
// }
//
// impl<'a, T> DoubleEndedIterator for MutColumnIterator<'a, T> {
//     fn next_back(&'a mut self) -> Option<Self::Item> {
//         let ll = self.grid[self.col].len();
//         if self.pos <= 0 {
//             return None;
//         }
//         let res = Some(&mut self.grid[self.col][self.pos]);
//         self.pos -= 1;
//         return res;
//     }
//
//     // fn next_back(&mut self) -> Option<Self::Item> {
//     //     if self.current_index > 0 {
//     //         self.current_index -= 1;
//     //         Some(&self.data[self.current_index])
//     //     } else {
//     //         None
//     //     }
//     // }
// }

// #[derive(Debug)]
// struct MutColumnIterator<'a, T, I>
// where
//     T: 'a
//     I: Iterator<Item = &'a mut Box<[T]>>,
// {
//     grid: I,
//     pos: usize,
//     col: usize,
// }
//
#[derive(Debug)]
struct MutColumnIterator<'a, T>
{
    grid: &'a mut [Vec<T>],
    pos: usize,
    col: usize,
}

impl<'a, T> MutColumnIterator<'a, T>
{
    fn new(grid: &'a mut [Vec<T>], col: usize) -> Self {
        MutColumnIterator {
            grid,
            pos: 0,
            col,
        }
    }
}

// impl <'a, T> Iterator for MutColumnIterator<'a, T>
// {
//     type Item = &'a mut T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let ll = self.grid[self.col].len();
//         if self.pos >= ll {
//             return None;
//         }
//         // let xx = self.grid[0]
//         let xx = &mut self.grid[self.col][self.pos];
//         // let xx = self.grid.get_mut(self.col).unwrap().get_mut(self.pos).unwrap();
//         self.pos += 1;
//         return Some(xx);
//     }
// }
//
// impl<'a, T, I> DoubleEndedIterator for MutColumnIterator<'a, T, I> {
//     fn next_back(&'a mut self) -> Option<Self::Item> {
//         let ll = self.grid[self.col].len();
//         if self.pos <= 0 {
//             return None;
//         }
//         let res = Some(&mut self.grid[self.col][self.pos]);
//         self.pos -= 1;
//         return res;
//     }
//
//     // fn next_back(&mut self) -> Option<Self::Item> {
//     //     if self.current_index > 0 {
//     //         self.current_index -= 1;
//     //         Some(&self.data[self.current_index])
//     //     } else {
//     //         None
//     //     }
//     // }
// }

fn visible_in_path<'a, I, I2>(iter: I, out_iter: I2)
    where
        I: Iterator<Item = &'a u8>,
        I: Clone,
        I2: Iterator<Item = &'a mut bool>,
{
    let path2 = iter.clone();
    let max_so_far = path2.scan(-1, |acc, c| {
        let ret = Some(*acc);
        *acc = max(*acc, *c as i64);
        ret
    }).collect::<Box<_>>();

    iter.zip(max_so_far.iter()).zip(out_iter).for_each(|((c, max), out)| {
        *out = *out || (*c as i64 > *max);
    });
}

fn max_height_until(trees: &Vec<Box<[u8]>>) -> u64 {
    let len = trees[0].len();
    // let mut vis_tree = Vec::new();
    let mut out: Vec<Vec<bool>> = trees.iter().map(|tree| {
        return tree.iter().map(|_| false).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    trees.iter().enumerate().for_each(|(idx, tree)| {
        visible_in_path(tree.iter(), out[idx].iter_mut());
        visible_in_path(tree.iter().rev(), out[idx].iter_mut().rev());
    });

    for i in 0..len {
        visible_in_path(ColumnIterator::new(trees, i), out.iter_mut().as_mut_slice().iter_mut().map(|c| c.get_mut(i).unwrap()));
        visible_in_path(ColumnIterator::new(trees, i).rev(), out.iter_mut().as_mut_slice().iter_mut().map(|c| c.get_mut(i).unwrap()).rev());
    }

    let s4: u64 = out.iter().map(|c| c.iter().map(|c| *c as u64).sum::<u64>()).sum();

    return s4
}

fn main() {
    let file = File::open("../input/y2022/d8.aoc.txt").unwrap();
    let mut string = String::new();
    std::io::BufReader::new(file).read_to_string(&mut string).unwrap();

    let mut rows = string.lines().peekable();

    let mut trees: Vec<Box<[u8]>> = Vec::new();
    for line in rows {
        let tree_row = line.chars().map(|c| (c.to_string()).parse().unwrap()).collect::<Box<_>>();
        trees.push(tree_row);
    }
    println!("trees: {:?}", trees);
    println!("sum vis {:?}", max_height_until(& trees));
    println!("best view score {:?}", best_view_score(&mut trees));
}

fn best_view_score(trees: &mut Vec<Box<[u8]>>) -> u64 {
    let mut best_score = 0;
    for i in 1..(trees.len()-1) {
        for j in 1..(trees[i].len()-1) {
            let height = trees[i][j];
            let right_score = viewable_in_dir(trees[i][(j+1)..].iter(), height);
            let left_score = viewable_in_dir(trees[i][..j].iter().rev(), height);
            let up_score = viewable_in_dir(ColumnIterator::new(&trees[..i], j).rev(), height);
            let down_score = viewable_in_dir(ColumnIterator::new(&trees[(i+1)..], j), height);
            let score = left_score * right_score * up_score * down_score;
            best_score = max(best_score, score);
        }
    }

    return best_score;
}

fn viewable_in_dir<'a, I>(path: I, height: u8) -> u64
    where I: Iterator<Item = &'a u8>
{
    return path.take_until(|p| *p >= height).count() as u64;
}

pub trait TakeUntilIterator<'a, I, P>: Iterator<Item = &'a u8> + Sized
    where I: Iterator<Item = &'a u8>,
          P: Fn(&'a u8) -> bool,

{
    fn take_until(self, p: P) -> TakeUntil<'a, Self, P> {
        TakeUntil::new(self, p)
    }
}

impl<'a, I: Iterator<Item = &'a u8>, P: Fn(&'a u8) -> bool> TakeUntilIterator<'a, I, P> for I {}



pub struct TakeUntil<'a, I, P>
    where I: Iterator<Item = &'a u8>,
          P: Fn(&'a u8) -> bool,

{
    iter: I,
    p: P,
    done: bool,
}


impl<'a, I, P> TakeUntil<'a, I, P>
    where I: Iterator<Item = &'a u8>,
        P: Fn(&'a u8) -> bool,
{
    fn new(iter: I, p: P) -> Self {
        TakeUntil {
            iter,
            p,
            done: false
        }
    }
}

impl <'a, I, P> Iterator for TakeUntil<'a, I, P>
    where I: Iterator<Item = &'a u8>,
          P: Fn(&'a u8) -> bool,
{
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if let Some(next) = self.iter.next() {
            if (self.p)(next) {
                self.done = true
            }
            return Some(next);
        }
        return None;
    }
}