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
            back_pos: grid[col].len(),
            col,
        }
    }
}

impl <'a, T> Iterator for ColumnIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let ll = self.grid[self.col].len();
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

    // out
    println!("max_so_far {:?}", max_so_far);
    println!("i1 {:?}", iter.clone().collect::<Vec<_>>());

    iter.zip(max_so_far.iter()).zip(out_iter).for_each(|((c, max), out)| {
        *out = *out || (*c as i64 > *max);
    });

    // return out_iter;
}

fn max_height_until(trees: &mut Vec<Box<[u8]>>) {
    let len = trees[0].len();
    // let mut vis_tree = Vec::new();
    let mut out: Vec<Vec<bool>> = trees.iter().map(|tree| {
        return tree.iter().map(|_| false).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    trees.iter().enumerate().for_each(|(idx, tree)| {
        visible_in_path(tree.iter(), out[idx].iter_mut());
    });

    let s1: u64 = out.iter().map(|c| c.iter().map(|c| *c as u64).sum::<u64>()).sum();


    trees.iter().enumerate().for_each(|(idx, tree)| {
        visible_in_path(tree.iter().rev(), out[idx].iter_mut().rev());
    });

    let s2: u64 = out.iter().map(|c| c.iter().map(|c| *c as u64).sum::<u64>()).sum();



    for i in 0..len {
        visible_in_path(ColumnIterator::new(trees, i), out.iter_mut().as_mut_slice().iter_mut().map(|c| c.get_mut(i).unwrap()));
    }

    let s3: u64 = out.iter().map(|c| c.iter().map(|c| *c as u64).sum::<u64>()).sum();


    for i in 0..len {
        visible_in_path(ColumnIterator::new(trees, i).rev(), out.iter_mut().as_mut_slice().iter_mut().map(|c| c.get_mut(i).unwrap()).rev());
    }

    let s4: u64 = out.iter().map(|c| c.iter().map(|c| *c as u64).sum::<u64>()).sum();

    // visible_in_path(ColumnIterator::new(trees, 1).rev(), out.iter_mut().as_mut_slice().iter_mut().map(|c| c.get_mut(1).unwrap()).rev());




    // let y= ;

    // let y = out.iter_mut().map(|x| {
    //    return &mut x[0]
    // });
    //
    // // let mut xx: MutColumnIterator<bool> = MutColumnIterator::new(&mut out, 0);
    // visible_in_path(ColumnIterator::new(trees, 0), y);

    // let y2 = out.iter_mut().map(|x| {
    //     return &mut x[0]
    // });

    // visible_in_path(ColumnIterator::new(trees, 0), y);
    //
    // visible_in_path(ColumnIterator::new(trees, 0).rev(), MutColumnIterator::new(&mut out, 0));
    // println!("out {:?}", out);

    println!("vis_tree {:?}", out);
    println!("s1 {:?}", s1);
    println!("s2 {:?}", s2);
    println!("s3 {:?}", s3);
    println!("s4 {:?}", s4);
    println!("x {:?}", true as u64);
    println!("x {:?}", false as u64);
}

fn main() {
    let file = File::open("../input/y2022/d8.aoc.txt").unwrap();
    let mut string = String::new();
    std::io::BufReader::new(file).read_to_string(&mut string).unwrap();

    let mut rows = string.lines().peekable();

    let width = rows.peek().unwrap().len();

    let mut trees: Vec<Box<[u8]>> = Vec::new();
    for line in rows {
        let tree_row = line.chars().map(|c| (c.to_string()).parse().unwrap()).collect::<Box<_>>();
        trees.push(tree_row);
    }
    println!("trees {:?}", trees);
    // println!("vis {:?}", visible_in_path(trees[1].iter()));
    println!("vis {:?}", max_height_until(&mut trees));
    // println!("here {:?}", trees.as_slice())
}